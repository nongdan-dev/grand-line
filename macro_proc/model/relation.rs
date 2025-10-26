use crate::prelude::*;

pub struct GenRelation {
    pub ty: RelationTy,
    pub ra: RelationAttr,
}

impl GenRelation {
    fn sql_dep_str(&self) -> String {
        match self.ty {
            RelationTy::BelongsTo => self.ra.key_str(),
            RelationTy::HasOne => s!("id"),
            RelationTy::HasMany => s!("id"),
            RelationTy::ManyToMany => s!("id"),
        }
    }
    fn input_one(&self) -> Ts2 {
        let mut inputs = ts2!();
        inputs = push_include_deleted(inputs, !self.ra.no_include_deleted);
        inputs
    }
    fn input_many(&self) -> Ts2 {
        let to = self.ra.to();
        let filter = ty_filter(&to);
        let order_by = ty_order_by(&to);
        let mut inputs = quote! {
            filter: Option<#filter>,
            order_by: Option<Vec<#order_by>>,
            page: Option<Pagination>,
        };
        inputs = push_include_deleted(inputs, !self.ra.no_include_deleted);
        inputs
    }

    fn output_one(&self) -> Ts2 {
        let to = self.ra.gql_to();
        quote!(Option<#to>)
    }
    fn output_many(&self) -> Ts2 {
        let to = self.ra.gql_to();
        quote!(Vec<#to>)
    }

    fn body_utils(&self, r: Ts2, vec: bool) -> Ts2 {
        let sql_dep = ts2!(self.sql_dep_str());
        let none = if vec { quote!(vec![]) } else { quote!(None) };
        quote! {
            if let Some(id) = self.#sql_dep.clone() {
                let _tx = ctx.tx().await?;
                let tx = _tx.as_ref();
                #r
            } else {
                #none
            }
        }
    }

    fn column(&self) -> Ts2 {
        ty_column(&self.ra.to())
    }
    fn col(&self) -> Ts2 {
        match self.ty {
            RelationTy::BelongsTo => pascal!("id"),
            RelationTy::HasOne => pascal!(self.ra.key_str()),
            RelationTy::HasMany => pascal!(self.ra.key_str()),
            RelationTy::ManyToMany => pascal!("id"),
        }
    }

    fn body_one(&self) -> Ts2 {
        let model = self.ra.to();
        let column = self.column();
        let col = self.col();
        let include_deleted = get_include_deleted(!self.ra.no_include_deleted);
        let r = quote! {
            #model::gql_load(ctx, #column::#col, id, #include_deleted).await?
        };
        self.body_utils(r, false)
    }
    fn body_many(&self, extra_cond: Ts2) -> Ts2 {
        let model = self.ra.to();
        let include_deleted = get_include_deleted(!self.ra.no_include_deleted);
        let r = quote! {
            #extra_cond
            #model::gql_search(ctx, tx, Some(extra_cond), filter, None, order_by, None, page, #include_deleted).await?
        };
        self.body_utils(r, true)
    }

    fn body_has_many(&self) -> Ts2 {
        let column = self.column();
        let col = self.col();
        let extra_cond = quote! {
            let extra_cond = Condition::all().add(#column::#col.eq(id));
        };
        self.body_many(extra_cond)
    }
    fn body_many_to_many(&self) -> Ts2 {
        let column = self.column();
        let col = self.col();
        let through = self.ra.through();
        let through_column = ty_column(&through);
        let through_key_col = pascal!(self.ra.key_str());
        let through_other_key_col = pascal!(self.ra.other_key());
        let extra_cond = quote! {
            let sub = #through::find()
                .select_only()
                .column(#through_column::#through_other_key_col)
                .filter(#through_column::#through_key_col.eq(id))
                .into_query();
            let extra_cond = Condition::all().add(#column::#col.in_subquery(sub));
        };
        self.body_many(extra_cond)
    }
}

impl VirtualResolverFn for GenRelation {
    fn sql_deps(&self) -> Vec<String> {
        vec![self.sql_dep_str()]
    }
}
impl AttrDebug for GenRelation {
    fn attr_debug(&self) -> String {
        self.ra.inner.attr_debug()
    }
}

impl ResolverFn for GenRelation {
    fn name(&self) -> Ts2 {
        self.ra.name()
    }
    fn gql_name(&self) -> String {
        camel_str!(self.name())
    }
    fn inputs(&self) -> Ts2 {
        match self.ty {
            RelationTy::BelongsTo => self.input_one(),
            RelationTy::HasOne => self.input_one(),
            RelationTy::HasMany => self.input_many(),
            RelationTy::ManyToMany => self.input_many(),
        }
    }
    fn output(&self) -> Ts2 {
        match self.ty {
            RelationTy::BelongsTo => self.output_one(),
            RelationTy::HasOne => self.output_one(),
            RelationTy::HasMany => self.output_many(),
            RelationTy::ManyToMany => self.output_many(),
        }
    }
    fn body(&self) -> Ts2 {
        match self.ty {
            RelationTy::BelongsTo => self.body_one(),
            RelationTy::HasOne => self.body_one(),
            RelationTy::HasMany => self.body_has_many(),
            RelationTy::ManyToMany => self.body_many_to_many(),
        }
    }
}
