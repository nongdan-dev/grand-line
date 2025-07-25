use crate::prelude::*;
use syn::Field;

pub struct GenRelation {
    pub model: String,
    pub ty: RelationTy,
    pub f: Field,
}

impl GenRelation {
    fn to(&self) -> TokenStream2 {
        self.f.ty.to_token_stream()
    }
    fn gql_to(&self) -> TokenStream2 {
        ty_gql(self.to())
    }

    fn key_str(&self) -> String {
        let default = match self.ty {
            RelationTy::BelongsTo => snake_str!(self.name(), "id"),
            _ => snake_str!(self.model, "id"),
        };
        self.attr_str("key", default)
    }
    fn other_key(&self) -> TokenStream2 {
        let k = "other_key";
        let default = match self.ty {
            RelationTy::ManyToMany => snake_str!(self.to(), "id"),
            _ => self.attr_panic(strf!("should not get {}", k)),
        };
        self.attr(k, default)
    }
    fn through(&self) -> TokenStream2 {
        let k = "through";
        let default = match self.ty {
            RelationTy::ManyToMany => pascal_str!(self.model, "In", self.to()),
            _ => self.attr_panic(strf!("should not get {}", k)),
        };
        self.attr(k, default)
    }

    fn input_one(&self) -> TokenStream2 {
        ts2!()
    }
    fn input_many(&self) -> TokenStream2 {
        let to = self.to();
        let filter = ty_filter(&to);
        let order_by = ty_order_by(&to);
        quote! {
            filter: Option<#filter>,
            order_by: Option<Vec<#order_by>>,
            page: Option<Pagination>,
        }
    }

    fn output_one(&self) -> TokenStream2 {
        ts2f!("Option<{}>", self.gql_to())
    }
    fn output_many(&self) -> TokenStream2 {
        ts2f!("Vec<{}>", self.gql_to())
    }

    fn body_utils(&self, r: TokenStream2) -> TokenStream2 {
        let id = ts2!(self.sql_dep());
        let err_str = strf!("{} must be included in the look ahead select", id);
        quote! {
            let id = self.#id.as_ref().ok_or(#err_str)?;
            let gl = GrandLineContext::from(ctx);
            let _tx = gl.tx().await?;
            let tx = _tx.as_ref();
            #r
        }
    }

    fn body_belongs_to(&self) -> TokenStream2 {
        let model = self.to();
        let column = ty_column(&model);
        let col = pascal!("Id");
        let r = quote! {
            let c = Condition::all().add(#column::#col.eq(id));
            #model::find().filter(c).gql_select(ctx).await?.one(tx).await?
        };
        self.body_utils(r)
    }
    fn body_has_one(&self) -> TokenStream2 {
        let model = self.to();
        let column = ty_column(&model);
        let col = pascal!(self.key_str());
        let r = quote! {
            let c = Condition::all().add(#column::#col.eq(id));
            #model::find().filter(c).gql_select(ctx).await?.one(tx).await?
        };
        self.body_utils(r)
    }
    fn body_has_many(&self) -> TokenStream2 {
        let model = self.to();
        let column = ty_column(&model);
        let col = pascal!(self.key_str());
        let r = quote! {
            let c = Condition::all().add(#column::#col.eq(id));
            #model::gql_search(ctx, tx, Some(c), filter, None, order_by, None, page).await?
        };
        self.body_utils(r)
    }
    fn body_many_to_many(&self) -> TokenStream2 {
        let model = self.to();
        let column = ty_column(&model);
        let through = self.through();
        let through_column = ty_column(&through);
        let through_key_col = pascal!(self.key_str());
        let through_other_key_col = pascal!(self.other_key());
        let r = quote! {
            let sub = #through::find()
                .select_only()
                .column(#through_column::#through_other_key_col)
                .filter(#through_column::#through_key_col.eq(id))
                .into_query();
            let c = Condition::all().add(#column::Id.in_subquery(sub));
            #model::gql_search(ctx, tx, Some(c), filter, None, order_by, None, page).await?
        };
        self.body_utils(r)
    }
}

impl GenVirtual for GenRelation {
    fn sql_dep(&self) -> String {
        match self.ty {
            RelationTy::BelongsTo => self.key_str(),
            RelationTy::HasOne => str!("id"),
            RelationTy::HasMany => str!("id"),
            RelationTy::ManyToMany => str!("id"),
        }
    }
}
impl DebugPanic for GenRelation {
    fn debug(&self) -> String {
        self.model.clone()
    }
}

impl GenResolverFn for GenRelation {
    fn name(&self) -> TokenStream2 {
        self.f.ident.to_token_stream()
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

impl MustGetAttr for GenRelation {
    fn impl_attr_model(&self) -> &dyn Display {
        &self.model
    }
    fn impl_attr_field(&self) -> &Field {
        &self.f
    }
    fn impl_attr_name(&self) -> &dyn Display {
        &self.ty
    }
}
