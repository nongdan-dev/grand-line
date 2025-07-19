use crate::prelude::*;
use syn::{Fields, ItemStruct, parse_macro_input};

pub fn gen_model(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut a = parse_attr!(attr);
    let mut struk = parse_macro_input!(item as ItemStruct);
    a.model = str!(struk.ident);
    // ------------------------------------------------------------------------
    // insert built-in fields: id, created_at, updated_at...
    struk = insert_builtin(&a, struk);
    // ------------------------------------------------------------------------
    // get the original model name, and set the new name that sea_orm requires
    // get the original model name in snake case for sql table, non-plural
    let alias = struk.ident;
    struk.ident = format_ident!("Model");
    let sql = ty_sql(&alias);
    let gql = ty_gql(&alias);
    let column = ty_column(&alias);
    let active_model = ty_active_model(&alias);
    let gql_alias = str!(alias);
    let sql_alias = snake_str!(alias);
    // ------------------------------------------------------------------------
    // filter / order_by fields
    let filter = ty_filter(&alias);
    let filter_combine = ty_filter_combine(&alias);
    let order_by = ty_order_by(&alias);
    let order_by_combine = ty_order_by_combine(&alias);
    let mut gql_struk = vec![];
    let mut gql_resolver = vec![];
    let mut gql_look_ahead = vec![];
    let mut gql_into = vec![];
    let mut filter_struk = vec![];
    let mut filter_query = vec![];
    let mut order_by_struk = vec![];
    let mut order_by_query = vec![];
    for ref f in parse_unwrap_ref!(struk.fields => Fields::Named)
        .named
        .iter()
    {
        push_gql(
            f,
            &mut gql_struk,
            &mut gql_resolver,
            &mut gql_look_ahead,
            &mut gql_into,
        );
        push_filter(f, &mut filter_struk, &mut filter_query);
        push_order_by(f, &mut order_by_struk, &mut order_by_query);
    }
    push_filter_and_or_not(
        &filter.to_token_stream(),
        &mut filter_struk,
        &mut filter_query,
    );

    quote! {
        use grand_line::*;
        use sea_orm::*;
        use sea_orm::entity::prelude::*;

        #[derive(
            Default,
            Clone,
            Debug,
            DeriveEntityModel,
            grand_line::macros::DeriveModel,
        )]
        #[sea_orm(table_name=#sql_alias)]
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
            #(#gql_struk)*
        }
        #[async_graphql::Object(name=#gql_alias)]
        impl #gql {
            #(#gql_resolver)*
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
            #(#filter_struk)*
        }
        impl Conditionable for #filter {
            fn condition(&self) -> Condition {
                let this = self.clone();
                let mut c = Condition::all();
                #(#filter_query)*
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
            #(#order_by_struk)*
        }
        impl Chainable<Entity> for #order_by {
            fn chain(&self, q: Select<Entity>) -> Select<Entity> {
                match *self {
                    #(#order_by_query)*
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
