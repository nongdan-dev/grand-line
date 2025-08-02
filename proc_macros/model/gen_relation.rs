use crate::prelude::*;

pub struct GenRelation {
    pub ty: RelationTy,
    pub a: RelationAttr,
}

impl GenRelation {
    fn sql_dep_(&self) -> String {
        match self.ty {
            RelationTy::BelongsTo => self.a.key_str(),
            RelationTy::HasOne => str!("id"),
            RelationTy::HasMany => str!("id"),
            RelationTy::ManyToMany => str!("id"),
        }
    }
    fn input_one(&self) -> TokenStream2 {
        ts2!()
    }
    fn input_many(&self) -> TokenStream2 {
        let to = self.a.to();
        let filter = ty_filter(&to);
        let order_by = ty_order_by(&to);
        quote! {
            filter: Option<#filter>,
            order_by: Option<Vec<#order_by>>,
            page: Option<Pagination>,
        }
    }

    fn output_one(&self) -> TokenStream2 {
        let to = self.a.gql_to();
        quote!(Option<#to>)
    }
    fn output_many(&self) -> TokenStream2 {
        let to = self.a.gql_to();
        quote!(Vec<#to>)
    }

    fn body_utils(&self, r: TokenStream2) -> TokenStream2 {
        let id = ts2!(self.sql_dep_());
        quote! {
            // TODO: handle case: original id is nullable Option<String>
            let id = self.#id.clone().ok_or_else(|| "should be selected from database already")?;
            let _tx = ctx.tx().await?;
            let tx = _tx.as_ref();
            #r
        }
    }

    fn column(&self) -> TokenStream2 {
        ty_column(&self.a.to())
    }
    fn col(&self) -> TokenStream2 {
        match self.ty {
            RelationTy::BelongsTo => pascal!("id"),
            RelationTy::HasOne => pascal!(self.a.key_str()),
            RelationTy::HasMany => pascal!(self.a.key_str()),
            RelationTy::ManyToMany => pascal!("id"),
        }
    }

    fn body_belongs_to(&self) -> TokenStream2 {
        let model = self.a.to();
        let column = self.column();
        let col = self.col();
        let r = quote! {
            let c = Condition::all().add(#column::#col.eq(id));
            #model::find().filter(c).gql_select(ctx).await?.one(tx).await?
        };
        self.body_utils(r)
    }
    fn body_has_one(&self) -> TokenStream2 {
        let model = self.a.to();
        let column = self.column();
        let col = self.col();
        let r = quote! {
            let c = Condition::all().add(#column::#col.eq(id));
            #model::find().filter(c).gql_select(ctx).await?.one(tx).await?
        };
        self.body_utils(r)
    }
    fn body_has_many(&self) -> TokenStream2 {
        let model = self.a.to();
        let column = self.column();
        let col = self.col();
        let r = quote! {
            let c = Condition::all().add(#column::#col.eq(id));
            #model::gql_search(ctx, tx, Some(c), filter, None, order_by, None, page).await?
        };
        self.body_utils(r)
    }
    fn body_many_to_many(&self) -> TokenStream2 {
        let model = self.a.to();
        let column = self.column();
        let col = self.col();
        let through = self.a.through();
        let through_column = ty_column(&through);
        let through_key_col = pascal!(self.a.key_str());
        let through_other_key_col = pascal!(self.a.other_key());
        let r = quote! {
            let sub = #through::find()
                .select_only()
                .column(#through_column::#through_other_key_col)
                .filter(#through_column::#through_key_col.eq(id))
                .into_query();
            let c = Condition::all().add(#column::#col.in_subquery(sub));
            #model::gql_search(ctx, tx, Some(c), filter, None, order_by, None, page).await?
        };
        self.body_utils(r)
    }
}

impl VirtualGen for GenRelation {
    fn sql_dep(&self) -> Vec<String> {
        vec![self.sql_dep_()]
    }
}
impl DebugPrefix for GenRelation {
    fn debug(&self) -> String {
        self.a.inner.debug()
    }
}

impl GenResolverFn for GenRelation {
    fn name(&self) -> TokenStream2 {
        self.a.name()
    }
    fn gql_name(&self) -> String {
        camel_str!(self.name())
    }
    fn inputs(&self) -> TokenStream2 {
        match self.ty {
            RelationTy::BelongsTo => self.input_one(),
            RelationTy::HasOne => self.input_one(),
            RelationTy::HasMany => self.input_many(),
            RelationTy::ManyToMany => self.input_many(),
        }
    }
    fn output(&self) -> TokenStream2 {
        match self.ty {
            RelationTy::BelongsTo => self.output_one(),
            RelationTy::HasOne => self.output_one(),
            RelationTy::HasMany => self.output_many(),
            RelationTy::ManyToMany => self.output_many(),
        }
    }
    fn body(&self) -> TokenStream2 {
        match self.ty {
            RelationTy::BelongsTo => self.body_belongs_to(),
            RelationTy::HasOne => self.body_has_one(),
            RelationTy::HasMany => self.body_has_many(),
            RelationTy::ManyToMany => self.body_many_to_many(),
        }
    }
}
