use crate::prelude::*;

#[cfg(feature = "authz")]
fn gen_row_f(filter: &Ts2) -> Ts2 {
    quote! { let _row_f = ctx.authz_row_graceful::<#filter>().await?; }
}
#[cfg(not(feature = "authz"))]
fn gen_row_f(filter: &Ts2) -> Ts2 {
    quote! { let _row_f: Option<#filter> = None; }
}

pub struct GenRelation {
    pub ty: RelationTy,
    pub a: RelationAttr,
    pub field_attrs: Vec<Attribute>,
}

impl GenRelation {
    fn sql_dep_str(&self) -> SynRes<String> {
        Ok(match self.ty {
            RelationTy::BelongsTo => self.a.key_str()?,
            RelationTy::HasOne => "id".to_owned(),
            RelationTy::HasMany => "id".to_owned(),
            RelationTy::ManyToMany => "id".to_owned(),
        })
    }
    fn input_one(&self) -> Ts2 {
        let mut inputs = quote!();
        inputs = push_include_deleted(inputs, self.a.include_deleted);
        inputs
    }
    fn input_many(&self) -> SynRes<Ts2> {
        let to = self.a.to()?;
        let filter = ty_filter(&to)?;
        let order_by = ty_order_by(&to)?;
        let mut inputs = quote! {
            filter: Option<#filter>,
            order_by: Option<Vec<#order_by>>,
            page: Option<Pagination>,
        };
        inputs = push_include_deleted(inputs, self.a.include_deleted);
        Ok(inputs)
    }

    fn output_one(&self) -> SynRes<Ts2> {
        let to = self.a.gql_to()?;
        Ok(quote!(Option<#to>))
    }
    fn output_many(&self) -> SynRes<Ts2> {
        let to = self.a.gql_to()?;
        Ok(quote!(Vec<#to>))
    }

    fn body_utils(&self, r: &Ts2, vec: bool) -> SynRes<Ts2> {
        let sql_dep = self.sql_dep_str()?.ts2_or_err()?;
        let none = if vec {
            quote!(vec![])
        } else {
            quote!(None)
        };
        Ok(quote! {
            if let Some(id) = self.#sql_dep.clone() {
                let tx = &*ctx.tx().await?;
                #r
            } else {
                #none
            }
        })
    }

    fn col(&self) -> SynRes<Ts2> {
        match self.ty {
            RelationTy::BelongsTo => "id".to_owned(),
            RelationTy::HasOne => self.a.key_str()?,
            RelationTy::HasMany => self.a.key_str()?,
            RelationTy::ManyToMany => "id".to_owned(),
        }
        .to_pascal_case()
        .ts2_or_err()
    }

    fn body_one(&self) -> SynRes<Ts2> {
        let model = self.a.to()?;
        let column = self.column()?;
        let col = self.col()?;
        let filter = ty_filter(&model)?;
        let include_deleted = get_include_deleted(self.a.include_deleted);
        let row_f = gen_row_f(&filter);
        let r = quote! {
            #row_f
            match _row_f {
                Some(_f) => {
                    let mut _q = #model::find().filter(#column::#col.eq(id.clone()));
                    if !#include_deleted.unwrap_or_default() {
                        _q = _q.exclude_deleted();
                    }
                    _q.chain(_f).gql_select(ctx)?.one(tx).await?
                }
                None => #model::gql_load(ctx, #column::#col, id, #include_deleted).await?,
            }
        };
        self.body_utils(&r, false)
    }
    fn body_has_many(&self) -> SynRes<Ts2> {
        let column = self.column()?;
        let col = self.col()?;
        let model = self.a.to()?;
        let filter = ty_filter(&model)?;
        let include_deleted = get_include_deleted(self.a.include_deleted);
        let row_f = gen_row_f(&filter);
        let extra_cond = quote! {
            let extra_cond = Condition::all().add(#column::#col.eq(id));
        };
        let r = quote! {
            #extra_cond
            #row_f
            #model::gql_search(ctx, tx, Some(extra_cond), filter, _row_f, order_by, None, page, #include_deleted).await?
        };
        self.body_utils(&r, true)
    }
    fn body_many_to_many(&self) -> SynRes<Ts2> {
        let column = self.column()?;
        let col = self.col()?;
        let through = self.a.through()?;
        let through_column = ty_column(&through)?;
        let through_key_col = self.a.key_str()?.to_pascal_case().ts2_or_err()?;
        let through_other_key_col = self.a.other_key()?.to_string().to_pascal_case().ts2_or_err()?;
        let model = self.a.to()?;
        let filter = ty_filter(&model)?;
        let include_deleted = get_include_deleted(self.a.include_deleted);
        let row_f = gen_row_f(&filter);
        let extra_cond = quote! {
            let sub = #through::find()
                .select_only()
                .column(#through_column::#through_other_key_col)
                .filter(#through_column::#through_key_col.eq(id))
                .into_query();
            let extra_cond = Condition::all().add(#column::#col.in_subquery(sub));
        };
        let r = quote! {
            #extra_cond
            #row_f
            #model::gql_search(ctx, tx, Some(extra_cond), filter, _row_f, order_by, None, page, #include_deleted).await?
        };
        self.body_utils(&r, true)
    }

    fn column(&self) -> SynRes<Ts2> {
        ty_column(self.a.to()?)
    }
}

impl VirtualResolverFn for GenRelation {
    fn sql_dep(&self) -> SynRes<Vec<String>> {
        Ok(vec![self.sql_dep_str()?])
    }
}
impl AttrDebug for GenRelation {
    fn attr_debug(&self) -> String {
        self.a.inner.attr_debug()
    }
    fn span(&self) -> Span {
        self.a.inner.span
    }
}

impl ResolverFn for GenRelation {
    fn name(&self) -> SynRes<Ts2> {
        self.a.name()
    }
    fn gql_name(&self) -> SynRes<String> {
        let (name_override, _) = attr_graphql_info(&self.field_attrs);
        if let Some(n) = name_override {
            return Ok(n);
        }
        Ok(self.name()?.to_string().to_lower_camel_case())
    }
    fn doc_strs(&self) -> Vec<String> {
        attr_doc_strs(&self.field_attrs)
    }
    fn extra_graphql(&self) -> Ts2 {
        attr_graphql_info(&self.field_attrs).1
    }
    fn inputs(&self) -> SynRes<Ts2> {
        match self.ty {
            RelationTy::BelongsTo => Ok(self.input_one()),
            RelationTy::HasOne => Ok(self.input_one()),
            RelationTy::HasMany => self.input_many(),
            RelationTy::ManyToMany => self.input_many(),
        }
    }
    fn output(&self) -> SynRes<Ts2> {
        match self.ty {
            RelationTy::BelongsTo => self.output_one(),
            RelationTy::HasOne => self.output_one(),
            RelationTy::HasMany => self.output_many(),
            RelationTy::ManyToMany => self.output_many(),
        }
    }
    fn body(&self) -> SynRes<Ts2> {
        match self.ty {
            RelationTy::BelongsTo => self.body_one(),
            RelationTy::HasOne => self.body_one(),
            RelationTy::HasMany => self.body_has_many(),
            RelationTy::ManyToMany => self.body_many_to_many(),
        }
    }
}
