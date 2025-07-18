use crate::prelude::*;
use heck::{ToLowerCamelCase, ToSnakeCase, ToUpperCamelCase};
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{ToTokens, format_ident, quote};
use syn::Field;
use syn::{Fields, ItemStruct, parse_macro_input};

pub fn gen_model(_attr: TokenStream, _item: TokenStream) -> TokenStream {
    let mut a = parse_attr!(_attr);
    let mut struk = parse_macro_input!(_item as ItemStruct);
    a.model = struk.ident.to_string();
    // ------------------------------------------------------------------------
    // get the original model name, and set the new name that sea_orm requires
    // get the original model name in snake case for sql table, non-plural
    let alias = struk.ident;
    struk.ident = format_ident!("Model");
    let sql = format_ident!("{}Sql", alias);
    let gql = format_ident!("{}Gql", alias);
    let column = format_ident!("{}Column", alias);
    let active_model = format_ident!("{}ActiveModel", alias);
    let alias_str = alias.to_string();
    let snake_str = alias_str.to_snake_case();
    // ------------------------------------------------------------------------
    // insert built-in fields: id, created_at, updated_at...
    struk = insert_builtin(&a, struk);
    // ------------------------------------------------------------------------
    // filter / order_by fields
    let filter = format_ident!("{}Filter", alias);
    let filter_combine = format_ident!("{}FilterCombine", alias);
    let order_by = format_ident!("{}OrderBy", alias);
    let order_by_combine = format_ident!("{}OrderByCombine", alias);
    let mut gql_fields = vec![];
    let mut gql_resolvers = vec![];
    let mut gql_look_ahead = vec![];
    let mut gql_into = vec![];
    let mut filter_fields = vec![];
    let mut filter_matches = vec![];
    let mut order_by_fields = vec![];
    let mut order_by_matches = vec![];
    for ref f in parse_unwrap_ref!(struk.fields => Fields::Named)
        .named
        .iter()
    {
        push_gql_fields(
            f,
            &mut gql_fields,
            &mut gql_resolvers,
            &mut gql_look_ahead,
            &mut gql_into,
        );
        push_filter_fields(f, &mut filter_fields, &mut filter_matches);
        push_order_by_fields(f, &mut order_by_fields, &mut order_by_matches);
    }
    push_filter_and_or_not(&filter, &mut filter_fields, &mut filter_matches);

    quote! {
        use grand_line::*;
        use sea_orm::*;
        use sea_orm::entity::prelude::*;

        #[derive(
            Default,
            Clone,
            Debug,
            DeriveEntityModel,
        )]
        #[sea_orm(table_name=#snake_str)]
        #struk

        impl ActiveModelBehavior for ActiveModel {
            fn new() -> Self {
                Self {
                    ..ActiveModelTrait::default()
                }
            }
        }
        #[derive(Debug, EnumIter, DeriveRelation)]
        pub enum Relation {
            // TODO:
        }

        pub type #sql = Model;
        pub type #alias = Entity;
        pub type #column = Column;
        pub type #active_model = ActiveModel;

        #[derive(
            Default,
            Clone,
            Debug,
            FromQueryResult,
        )]
        pub struct #gql {
            #(#gql_fields)*
        }
        #[async_graphql::Object(name=#alias_str)]
        impl #gql {
            #(#gql_resolvers)*
        }
        impl Entity {
            /// select only columns that in the request context
            fn gql_select(ctx: &async_graphql::Context<'_>, mut q: Select<Entity>) -> Selector<SelectModel<#gql>> {
                q = q.select_only();
                let l = ctx.look_ahead();
                #(#gql_look_ahead)*
                q.into_model::<#gql>()
            }
            /// select only id for the delete result
            fn gql_select_id(ctx: &async_graphql::Context<'_>, q: Select<Entity>) -> Selector<SelectModel<#gql>> {
                q.select_only().column(Column::Id).into_model::<#gql>()
            }
        }
        impl From<#sql> for #gql {
            fn from(v: #sql) -> Self {
                #gql {
                    #(#gql_into)*
                    ..Default::default()
                }
            }
        }

        #[input]
        pub struct #filter {
            #(#filter_fields)*
        }
        impl Conditionable for #filter {
            fn condition(&self) -> Condition {
                let this = self.clone();
                let mut c = Condition::all();
                #(#filter_matches)*
                c
            }
        }
        impl Chainable<Entity> for #filter {
            fn chain(&self, q: Select<Entity>) -> Select<Entity> {
                q.filter(self.condition())
            }
        }
        /**
         * Helper to combine filter and extra_filter
         */
        pub trait #filter_combine {
            fn combine(self, e: Option<#filter>) -> Option<#filter>;
        }
        impl #filter_combine for Option<#filter> {
            fn combine(self, e: Option<#filter>) -> Option<#filter> {
                CrudHelpers::filter_combine(self, e, &|a, b| #filter {
                    and: Some(vec![a, b]),
                    ..Default::default()
                })
            }
        }

        #[derive(
          Clone,
          Debug,
          Copy,
          Eq,
          PartialEq,
          serde::Deserialize,
          serde::Serialize,
          async_graphql::Enum,
        )]
        pub enum #order_by {
            #(#order_by_fields)*
        }
        impl Chainable<Entity> for #order_by {
            fn chain(&self, q: Select<Entity>) -> Select<Entity> {
                match *self {
                    #(#order_by_matches)*
                }
            }
        }

        /**
         * Helper to combine order_by and default_order_by with an initial value if all are empty
         */
        pub trait #order_by_combine {
            fn combine(self, d: Option<Vec<#order_by>>) -> Vec<#order_by>;
        }
        impl #order_by_combine for Option<Vec<#order_by>> {
            fn combine(self, d: Option<Vec<#order_by>>) -> Vec<#order_by> {
                CrudHelpers::order_by_combine(self, d, #order_by::IdDesc)
            }
        }

        impl Entity {
            pub async fn gql_search(
                ctx: &async_graphql::Context<'_>,
                tx: &DatabaseTransaction,
                filter: Option<#filter>,
                extra_filter: Option<#filter>,
                order_by: Option<Vec<#order_by>>,
                default_order_by: Option<Vec<#order_by>>,
                page: Option<Pagination>,
            ) -> Result<Vec<#gql>, Box<dyn std::error::Error + Send + Sync>> {
                let q = filter.combine(extra_filter).query();
                let q = order_by.combine(default_order_by).chain(q);
                let (offset, limit) = CrudHelpers::pagination(page, 100, 1000);
                let q = q.offset(offset).limit(limit);
                let q = Entity::gql_select(ctx, q);
                Ok(q.all(tx).await?)
            }
            pub async fn gql_count(
                ctx: &async_graphql::Context<'_>,
                tx: &DatabaseTransaction,
                filter: Option<#filter>,
                extra_filter: Option<#filter>,
            ) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
                let q = filter.combine(extra_filter).query();
                Ok(q.count(tx).await?)
            }
            pub async fn gql_detail(
                ctx: &async_graphql::Context<'_>,
                tx: &DatabaseTransaction,
                id: &String,
            ) -> Result<#gql, Box<dyn std::error::Error + Send + Sync>> {
                let q = Entity::find_by_id(id);
                let q = Entity::gql_select(ctx, q);
                match q.one(tx).await? {
                    Some(v) => Ok(v),
                    None => Err("404".into()),
                }
            }
            pub async fn gql_delete(
                ctx: &async_graphql::Context<'_>,
                tx: &DatabaseTransaction,
                id: &String,
            ) -> Result<Option<#gql>, Box<dyn std::error::Error + Send + Sync>> {
                let q = Entity::find_by_id(id);
                let q = Entity::gql_select_id(ctx, q);
                match q.one(tx).await? {
                    Some(v) => {
                        Entity::delete_by_id(id).exec(tx).await?;
                        Ok(Some(v))
                    }
                    None => Ok(None),
                }
            }
        }
    }
    .into()
}

fn push_gql_fields(
    f: &Field,
    fields: &mut Vec<TokenStream2>,
    resolvers: &mut Vec<TokenStream2>,
    look_ahead: &mut Vec<TokenStream2>,
    into: &mut Vec<TokenStream2>,
) {
    let name = f.ident.clone();
    let name_str = name.to_token_stream().to_string();
    let camel_str = name_str.to_lower_camel_case();
    let ty = &f.ty;
    let (opt, ty_str) = unwrap_option(ty.to_token_stream());

    let ty_unwrapped: TokenStream2 = ty_str.parse().unwrap();
    fields.push(quote! {
        #name: Option<#ty_unwrapped>,
    });

    let res = if opt {
        quote! {
            self.#name
        }
    } else {
        quote! {
            self.#name.clone().unwrap_or_default()
        }
    };
    resolvers.push(quote! {
        #[graphql(name=#camel_str)]
        async fn #name(&self) -> #ty {
            #res
        }
    });

    let pascal: TokenStream2 = name_str.to_upper_camel_case().parse().unwrap();
    look_ahead.push(quote! {
        if l.field(#camel_str).exists() {
            q = q.column(Column::#pascal)
        }
    });

    let res = if opt {
        quote! {
            #name: v.#name,
        }
    } else {
        quote! {
            #name: Some(v.#name),
        }
    };
    into.push(quote! {
        #res
    });
}
