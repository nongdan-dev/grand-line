use crate::prelude::*;
use syn::{Fields, ItemStruct, parse_macro_input};

pub fn gen_model(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut a = parse_macro_input!(attr as MacroAttr);
    let mut struk = parse_macro_input!(item as ItemStruct);
    a.model = str!(struk.ident);
    // ------------------------------------------------------------------------
    // insert built-in fields: id, created_at, updated_at...
    struk = insert_builtin(&a, struk);
    // ------------------------------------------------------------------------
    // extract virtual fields such as relation and so on...
    let mut relation_fields = vec![];
    let mut relation_dep_sql = vec![];
    struk.fields = match struk.fields {
        Fields::Named(ref mut f) => {
            f.named = f
                .named
                .clone()
                .into_iter()
                .filter(|f| {
                    let ref mut attrs = f.attrs.iter().map(|a| a.path());
                    let relation = if attrs.any(|p| p.is_ident("belongs_to")) {
                        "belongs_to"
                    } else if attrs.any(|p| p.is_ident("has_one")) {
                        "has_one"
                    } else if attrs.any(|p| p.is_ident("has_many")) {
                        "has_many"
                    } else if attrs.any(|p| p.is_ident("many_to_many")) {
                        "many_to_many"
                    } else {
                        ""
                    };
                    if relation != "" {
                        relation_fields.push((relation, f.clone()));
                        if relation == "belongs_to" {
                            relation_dep_sql.push(snake_str!(f.ident.to_token_stream(), "id"));
                        } else {
                            panic!("TODO:");
                        }
                        return false;
                    }
                    true
                })
                .collect();
            Fields::Named(f.clone())
        }
        _ => panic!(
            "{} fields must be Fields::Named, found {}",
            a.model,
            struk.fields.to_token_stream()
        ),
    };
    let relation_dep_gql = relation_fields
        .iter()
        .map(|f| str!(f.1.ident.to_token_stream()))
        .collect();
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
    let order_by = ty_order_by(&alias);
    let mut gql_struk = vec![];
    let mut gql_resolver = vec![];
    let mut gql_into = vec![];
    let mut gql_columns = vec![];
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
            &relation_dep_sql,
            &relation_dep_gql,
            &mut gql_struk,
            &mut gql_resolver,
            &mut gql_into,
            &mut gql_columns,
        );
        push_filter(f, &mut filter_struk, &mut filter_query);
        push_order_by(f, &mut order_by_struk, &mut order_by_query);
    }
    push_filter_and_or_not(&filter, &mut filter_struk, &mut filter_query);

    for f in relation_fields {
        let f = &f.1;
        let name = f.ident.to_token_stream();
        let gql_name = camel_str!(name);
        let fkey = snake!(name, "id");
        let model = f.ty.clone().into_token_stream();
        let ty = ts2!("Option<", model, "Gql>");
        gql_resolver.push(quote! {
            #[graphql(name=#gql_name)]
            async fn #name(&self, ctx: &async_graphql::Context<'_>) -> Result<#ty, Box<dyn Error + Send + Sync>> {
                // TODO: data loader
                let gl = GrandLineContext::from(ctx);
                let _tx = gl.tx().await?;
                let tx = _tx.as_ref();
                let q = #model::find_by_id(self.#fkey.clone().unwrap_or_default());
                let r = #model::gql_select(ctx, q).one(tx).await?;
                Ok(r)
            }
        });
    }

    let am_id = quote! {
        if !matches!(am.id, Set(_)) {
            am.id = Set(ulid::Ulid::new().to_string());
        }
    };
    let am_created_at = if a.no_created_at {
        ts2!("")
    } else {
        quote! {
            if !matches!(am.created_at, ActiveValue::Set(_)) {
                am.created_at = ActiveValue::Set(chrono::Utc::now());
            }
        }
    };
    let am_updated_at = if a.no_updated_at {
        ts2!("")
    } else {
        quote! {
            if !matches!(am.updated_at, ActiveValue::Set(_)) {
                am.updated_at = ActiveValue::Set(Some(chrono::Utc::now()));
            }
        }
    };

    quote! {
        use sea_orm::*;
        use sea_orm::prelude::*;
        use sea_orm::entity::prelude::*;

        #[derive(
            Default,
            Clone,
            Debug,
            DeriveEntityModel,
            GrandLineModel,
        )]
        #[sea_orm(table_name=#sql_alias)]
        #struk

        impl ActiveModelBehavior for ActiveModel {
        }
        impl Entity {
            /// sea_orm ActiveModel hooks will not be called with Entity:: or bulk methods.
            /// We need to have this method instead to get default values on create.
            /// This can be used together with the macro grand_line::active_create
            pub fn active_create(mut am: ActiveModel) -> ActiveModel {
                #am_id
                #am_created_at
                am
            }
            /// sea_orm ActiveModel hooks will not be called with Entity:: or bulk methods.
            /// We need to have this method instead to get default values on update.
            /// This can be used together with the macro grand_line::active_update
            pub fn active_update(mut am: ActiveModel) -> ActiveModel {
                #am_updated_at
                am
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
        impl From<#sql> for #gql {
            fn from(v: #sql) -> Self {
                #gql {
                    #(#gql_into)*
                    ..Default::default()
                }
            }
        }

        static GQL_COLUMNS: once_cell::sync::Lazy<std::collections::HashMap<&'static str, Column>> = once_cell::sync::Lazy::new(|| {
            let mut m = std::collections::HashMap::new();
            #(#gql_columns)*
            m
        });
        impl Gql<Entity, #filter, #order_by, #gql> for Entity {
            fn id() -> Column {
                Column::Id
            }
            fn column(field: &str) -> Option<Column> {
                GQL_COLUMNS.get(field).copied()
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
        impl Filter for #filter {
            fn combine(a: Self, b: Self) -> Self {
                Self {
                    and: Some(vec![a, b]),
                    ..Default::default()
                }
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
            /// Helper to concat sea_orm select query with order_by
            fn chain(&self, q: Select<Entity>) -> Select<Entity> {
                match *self {
                    #(#order_by_query)*
                }
            }
        }
        impl OrderBy for #order_by {
            fn default() -> Self {
                Self::IdDesc
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
            ) -> Result<Vec<#gql>, Box<dyn Error + Send + Sync>> {
                let q = filter.combine(extra_filter).query();
                let q = order_by.combine(default_order_by).chain(q);
                // TODO: config 100, 1000 globally, and via model attr
                let (offset, limit) = pagination(page, 100, 1000);
                let q = q.offset(offset).limit(limit);
                let q = Entity::gql_select(ctx, q);
                Ok(q.all(tx).await?)
            }
            pub async fn gql_count(
                ctx: &async_graphql::Context<'_>,
                tx: &DatabaseTransaction,
                filter: Option<#filter>,
                extra_filter: Option<#filter>,
            ) -> Result<u64, Box<dyn Error + Send + Sync>> {
                let q = filter.combine(extra_filter).query();
                Ok(q.count(tx).await?)
            }
            pub async fn gql_detail(
                ctx: &async_graphql::Context<'_>,
                tx: &DatabaseTransaction,
                id: &String,
            ) -> Result<#gql, Box<dyn Error + Send + Sync>> {
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
            ) -> Result<Option<#gql>, Box<dyn Error + Send + Sync>> {
                let q = Entity::find_by_id(id);
                let q = Entity::gql_select_id(q);
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
