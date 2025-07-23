use crate::prelude::*;
use std::str::FromStr;
use syn::{Fields, ItemStruct, parse_macro_input};

pub fn gen_model(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut a = parse_macro_input!(attr as MacroAttr);
    let mut struk = parse_macro_input!(item as ItemStruct);
    a.model = str!(struk.ident);
    let ref mut fields = match struk.fields {
        Fields::Named(f) => f,
        _ => panic!("{} struct fields must be Fields::Named", a.model),
    };
    // ------------------------------------------------------------------------
    // insert built-in fields: id, created_at, updated_at...
    fields.named.insert(
        0,
        field! {
            #[sea_orm(primary_key, column_type="String(StringLen::N(26))", auto_increment=false)]
            pub id: String
        },
    );
    if !a.no_created_at {
        fields.named.push(field! {
            pub created_at: DateTimeUtc
        });
        if !a.no_by_id {
            fields.named.push(field! {
                pub created_by_id: Option<String>
            });
        }
    }
    if !a.no_updated_at {
        fields.named.push(field! {
            pub updated_at: Option<DateTimeUtc>
        });
        if !a.no_by_id {
            fields.named.push(field! {
                pub updated_by_id: Option<String>
            });
        }
    }
    if !a.no_deleted_at {
        fields.named.push(field! {
            pub deleted_at: Option<DateTimeUtc>
        });
        if !a.no_by_id {
            fields.named.push(field! {
                pub deleted_by_id: Option<String>
            });
        }
    }
    // ------------------------------------------------------------------------
    // extract virtual fields such as relation and so on...
    let mut virtuals = vec![];
    fields.named = fields
        .named
        .clone()
        .into_iter()
        .filter(|f| {
            let relation = vec![
                RelationTy::BelongsTo,
                RelationTy::HasOne,
                RelationTy::HasMany,
                RelationTy::ManyToMany,
            ]
            .iter()
            .map(|r| str!(r))
            .find(|r| is_attr(f, &r));
            if let Some(relation) = relation {
                virtuals.push(Relation {
                    ty: RelationTy::from_str(&relation).unwrap(),
                    f: f.clone(),
                });
                return false;
            }
            // TODO: other kinds of virtual
            true
        })
        .collect();
    // ------------------------------------------------------------------------
    // get the original model name, and set the new name that sea_orm requires
    // get the original model name in snake case for sql table, non-plural
    let alias = struk.ident;
    struk.ident = format_ident!("Model");
    struk.fields = Fields::Named(fields.clone());
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
    for f in fields.named.iter() {
        push_gql(
            f,
            &virtuals,
            &mut gql_struk,
            &mut gql_resolver,
            &mut gql_into,
            &mut gql_columns,
        );
        push_filter(f, &mut filter_struk, &mut filter_query);
        push_order_by(f, &mut order_by_struk, &mut order_by_query);
    }
    push_filter_and_or_not(&filter, &mut filter_struk, &mut filter_query);

    for f in virtuals.iter() {
        if matches!(f.ty, RelationTy::BelongsTo | RelationTy::HasOne) {
            let model = f.model();
            let gql = ts2!("Option<", f.gql(), ">");
            let name = f.name();
            let gql_name = f.gql_name();
            let id = f.id();
            let column = ty_column(&model);
            let col = f.column();
            gql_resolver.push(quote! {
                #[graphql(name=#gql_name)]
                async fn #name(&self, ctx: &async_graphql::Context<'_>) -> Result<#gql, Box<dyn Error + Send + Sync>> {
                    // TODO: data loader: belongs to then find by id, otherwise find by fkey column
                    let gl = GrandLineContext::from(ctx);
                    let _tx = gl.tx().await?;
                    let tx = _tx.as_ref();
                    let id = self.#id.clone().unwrap_or_default();
                    let c = Condition::all().add(#column::#col.eq(id));
                    let q = #model::find().filter(c);
                    let r = #model::gql_select(ctx, q).await?.one(tx).await?;
                    Ok(r)
                }
            });
        } else if matches!(f.ty, RelationTy::HasMany) {
            let model = f.model();
            let gql = ts2!("Vec<", f.gql(), ">");
            let name = f.name();
            let gql_name = f.gql_name();
            let id = f.id();
            let column = ty_column(&model);
            let col = f.column();
            let filter = ty_filter(&model);
            let order_by = ty_order_by(&model);
            let db_fn = ts2!(&model, "::gql_search");
            gql_resolver.push(quote! {
                #[graphql(name=#gql_name)]
                async fn #name(
                    &self,
                    ctx: &async_graphql::Context<'_>,
                    filter: Option<#filter>,
                    order_by: Option<Vec<#order_by>>,
                    page: Option<Pagination>,
                ) -> Result<#gql, Box<dyn Error + Send + Sync>> {
                    let gl = GrandLineContext::from(ctx);
                    let _tx = gl.tx().await?;
                    let tx = _tx.as_ref();
                    let id = self.#id.clone().unwrap_or_default();
                    let c = Condition::all().add(#column::#col.eq(id));
                    let r = #db_fn(ctx, tx, Some(c), filter, None, order_by, None, page).await?;
                    Ok(r)
                }
            });
        } else {
            panic!("TODO:");
        }
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
            /// This can be used together with the macro grand_line::active_create.
            pub fn active_create(mut am: ActiveModel) -> ActiveModel {
                #am_id
                #am_created_at
                am
            }
            /// sea_orm ActiveModel hooks will not be called with Entity:: or bulk methods.
            /// We need to have this method instead to get default values on update.
            /// This can be used together with the macro grand_line::active_update.
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
        impl EntityX<Model, #filter, #order_by, #gql> for Entity {
            fn id() -> Column {
                Column::Id
            }
            fn column(field: &str) -> Option<Column> {
                GQL_COLUMNS.get(field).copied()
            }
            // TODO: config_limit via model attr
        }

        #[input]
        pub struct #filter {
            #(#filter_struk)*
        }
        impl Filter<Entity> for #filter {
            fn combine(a: Self, b: Self) -> Self {
                Self {
                    and: Some(vec![a, b]),
                    ..Default::default()
                }
            }
        }
        impl Conditionable for #filter {
            fn condition(&self) -> Condition {
                let this = self.clone();
                let mut c = Condition::all();
                #(#filter_query)*
                c
            }
        }

        #[enunn]
        pub enum #order_by {
            #(#order_by_struk)*
        }
        impl OrderBy<Entity> for #order_by {
            fn default() -> Self {
                Self::IdDesc
            }
        }
        impl Chainable<Entity> for #order_by {
            fn chain(&self, q: Select<Entity>) -> Select<Entity> {
                match *self {
                    #(#order_by_query)*
                }
            }
        }
    }
    .into()
}
