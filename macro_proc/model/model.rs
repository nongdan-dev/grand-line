use crate::prelude::*;
use syn::{Fields, ItemStruct, parse_macro_input};

pub fn gen_model(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr = parse_macro_input!(attr as AttrParse);
    let mut item = parse_macro_input!(item as ItemStruct);
    let a = attr.into_inner::<ModelAttr>("model");
    // ------------------------------------------------------------------------
    // insert built-in fields: id, created_at, updated_at...
    let model = item.ident.to_token_stream();
    let mut fields = match item.fields {
        Fields::Named(f) => f.named,
        _ => {
            let err = f!("{} struct should be named fields", model);
            let err = a.inner.err(&err);
            pan!(err);
        }
    };
    fields.insert(
        0,
        field! {
            #[sea_orm(primary_key, column_type="String(StringLen::N(26))", auto_increment=false)]
            pub id: String
        },
    );
    if !a.no_created_at {
        fields.push(field!(pub created_at: DateTimeUtc));
        if !a.no_by_id {
            fields.push(field!(pub created_by_id: Option<String>));
        }
    }
    if !a.no_updated_at {
        fields.push(field!(pub updated_at: Option<DateTimeUtc>));
        if !a.no_by_id {
            fields.push(field!(pub updated_by_id: Option<String>));
        }
    }
    let mut config_has_deleted_at = quote!(false);
    let mut sql_soft_delete = ts2!();
    let mut am_soft_delete_impl = ts2!();
    let mut am_soft_delete = ts2!();
    if !a.no_deleted_at {
        fields.push(field!(pub deleted_at: Option<DateTimeUtc>));
        if !a.no_by_id {
            fields.push(field!(pub deleted_by_id: Option<String>));
        }
        config_has_deleted_at = quote! {
            !self.deleted_at.is_undefined() ||
            !self.deleted_at_ne.is_undefined() ||
            self.deleted_at_in.is_some() ||
            self.deleted_at_not_in.is_some() ||
            self.deleted_at_gt.is_some() ||
            self.deleted_at_gte.is_some() ||
            self.deleted_at_lt.is_some() ||
            self.deleted_at_lte.is_some()
        };
        sql_soft_delete = quote! {
            pub fn soft_delete_by_id(id: &str) -> Result<ActiveModel, Box<dyn Error + Send + Sync>> {
                let c = Self::condition_id(id)?;
                let mut am = am_update!(#model {
                    id: id.to_string(),
                });
                am.deleted_at = am.updated_at.clone();
                Ok(am)
            }
        };
        am_soft_delete_impl = quote! {
            async fn soft_delete<D>(self, db: &D) -> Result<Model, Box<dyn Error + Send + Sync>>
            where
                D: ConnectionTrait;
        };
        am_soft_delete = quote! {
            async fn soft_delete<D>(self, db: &D) -> Result<Model, Box<dyn Error + Send + Sync>>
            where
                D: ConnectionTrait,
            {
                let mut am = am_update!(#model {
                    ..self
                });
                am.deleted_at = am.updated_at.clone();
                let r = am.update(db).await?;
                Ok(r)
            }
        };
    }
    // ------------------------------------------------------------------------
    // parse macro attributes, extract and validate fields
    let model_str = s!(model);
    let (defs, virs, exprs, gfields, fields) = attr_extract(&model_str, &fields);
    // ------------------------------------------------------------------------
    // get the original model name, and set the new name that sea_orm requires
    // get the original model name in snake case for sql table, non-plural
    item.ident = format_ident!("Model");
    item.fields = Fields::Named(fields);
    let module = snake!(model);
    let sql = ty_sql(&model);
    let gql = ty_gql(&model);
    let column = ty_column(&model);
    let active_model = ty_active_model(&model);
    let active_model_async_impl = ty_active_model_async_impl(&model);
    let gql_alias = s!(model);
    let sql_alias = snake_str!(model);
    // ------------------------------------------------------------------------
    // generate virtual resolvers
    let mut vgens = Vec::<Box<dyn VirtualResolverFn>>::new();
    for attrs in virs {
        let map = attrs
            .into_iter()
            .map(|a| (a.attr.clone(), a))
            .collect::<HashMap<_, _>>();
        for (a, v) in VirtualTy::all()
            .iter()
            .filter_map(|v| map.get(&s!(v)).map(move |a| (a, v)))
        {
            vgens.push(match v {
                VirtualTy::Relation(ty) => Box::new(GenRelation {
                    ty: ty.clone(),
                    a: a.clone().into_with_validate(),
                }),
                VirtualTy::Resolver => Box::new(GenResolver {
                    a: a.clone().into_with_validate(),
                }),
                _ => {
                    let err = f!("invalid attr={} dyn VirtualGen", a.attr);
                    bug!(err);
                }
            });
        }
    }
    // ------------------------------------------------------------------------
    // filter / order_by fields
    let (mut filter_struk, mut filter_query) = (vec![], vec![]);
    let (mut order_by_struk, mut order_by_query) = (vec![], vec![]);
    for (f, _) in &gfields {
        filter(f, &mut filter_struk, &mut filter_query);
        order_by(f, &mut order_by_struk, &mut order_by_query);
    }
    let filter = ty_filter(&model);
    let order_by = ty_order_by(&model);
    filter_and_or_not(&filter, &mut filter_struk, &mut filter_query);
    // ------------------------------------------------------------------------
    // gql fields
    let (mut gql_struk, mut gql_resolver, gql_into, sql_cols) = gql_fields(&gfields);
    let mut gql_select = gql_virtuals(&vgens);
    let (gql_struk2, gql_resolver2, gql_select2, sql_exprs) = gql_exprs(&exprs);
    gql_struk.extend(gql_struk2);
    gql_resolver.extend(gql_resolver2);
    gql_select.extend(gql_select2);
    for f in vgens {
        gql_resolver.push(f.resolver_fn());
    }
    // ------------------------------------------------------------------------
    // active model default
    let mut am_defs = vec![];
    for a in defs {
        let mut raw_str = a.raw();
        if raw_str.starts_with("\"") || raw_str.starts_with("r#") {
            raw_str = raw_str + ".to_string()"
        }
        let raw = ts2!(raw_str);
        let name = ts2!(a.field_name());
        am_defs.push(quote! {
            if !matches!(am.#name, Set(_)) {
                am.#name = Set(#raw);
            }
        });
    }
    // ------------------------------------------------------------------------
    // active model utils
    let am_id = quote! {
        if !matches!(am.id, Set(_)) {
            am.id = Set(ulid::Ulid::new().to_string());
        }
    };
    let am_created_at = if a.no_created_at {
        ts2!()
    } else {
        quote! {
            if !matches!(am.created_at, Set(_)) {
                am.created_at = Set(chrono::Utc::now());
            }
        }
    };
    let am_updated_at = if a.no_updated_at {
        ts2!()
    } else {
        quote! {
            if !matches!(am.updated_at, Set(_)) {
                am.updated_at = Set(Some(chrono::Utc::now()));
            }
        }
    };
    // ------------------------------------------------------------------------
    // config limit
    let (limit_default, limit_max) = (a.limit_default, a.limit_max);
    let config_limit = quote! {
        ConfigLimit {
            default: #limit_default,
            max: #limit_max,
        }
    };

    let r = quote! {
        pub mod #module {
            use super::*;
            use sea_orm::*;
            use sea_orm::prelude::*;
            use sea_orm::entity::prelude::*;

            #[derive(
                Debug,
                Clone,
                Default,
                DeriveEntityModel,
            )]
            #[sea_orm(table_name=#sql_alias)]
            #item

            impl ActiveModelBehavior for ActiveModel {
            }

            #[derive(Debug, EnumIter, DeriveRelation)]
            pub enum Relation {
                // TODO:
            }

            #[derive(
                Debug,
                Clone,
                Default,
                FromQueryResult,
            )]
            pub struct #gql {
                #(#gql_struk)*
            }
            #[async_graphql::Object(name=#gql_alias)]
            impl #gql {
                #(#gql_resolver)*
            }
            impl From<Model> for #gql {
                fn from(v: Model) -> Self {
                    #gql {
                        #(#gql_into)*
                        ..Default::default()
                    }
                }
            }

            impl Entity {
                #sql_soft_delete
            }

            static SQL_COLS: LazyLock<HashMap<&'static str, Column>> = LazyLock::new(|| {
                let mut m = HashMap::new();
                #(#sql_cols)*
                m
            });
            static SQL_EXPRS: LazyLock<HashMap<&'static str, sea_query::SimpleExpr>> = LazyLock::new(|| {
                let mut m = HashMap::new();
                #(#sql_exprs)*
                m
            });
            static GQL_SELECT: LazyLock<HashMap<&'static str, Vec<&'static str>>> = LazyLock::new(|| {
                let mut m = HashMap::new();
                #(#gql_select)*
                m
            });

            impl EntityX<Model, ActiveModel, #filter, #order_by, #gql> for Entity {
                fn config_limit() -> ConfigLimit {
                    #config_limit
                }
                fn config_am_create(mut am: ActiveModel) -> ActiveModel {
                    #am_id
                    #am_created_at
                    #(#am_defs)*
                    am
                }
                fn config_am_update(mut am: ActiveModel) -> ActiveModel {
                    #am_updated_at
                    am
                }
                fn config_sql_cols() -> &'static LazyLock<HashMap<&'static str, Self::Column>> {
                    &SQL_COLS
                }
                fn config_sql_exprs() -> &'static LazyLock<HashMap<&'static str, sea_query::SimpleExpr>> {
                    &SQL_EXPRS
                }
                fn config_gql_select() -> &'static LazyLock<HashMap<&'static str, Vec<&'static str>>> {
                    &GQL_SELECT
                }
            }

            #[async_trait]
            pub trait ActiveModelAsyncImpl {
                #am_soft_delete_impl
            }

            #[async_trait]
            impl ActiveModelAsyncImpl for ActiveModel {
                #am_soft_delete
            }

            #[gql_input]
            pub struct #filter {
                #(#filter_struk)*
            }
            impl Filter<Entity> for #filter {
                fn config_and(a: Self, b: Self) -> Self {
                    Self {
                        and: Some(vec![a, b]),
                        ..Default::default()
                    }
                }
                fn config_has_deleted_at(&self) -> bool {
                    #config_has_deleted_at
                }
                fn and(&self) -> Option<Vec<Self>> {
                    self.and.clone()
                }
                fn or(&self) -> Option<Vec<Self>> {
                    self.or.clone()
                }
                fn not(&self) -> Option<Self> {
                    self.not.clone().map(|b| *b)
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

            #[gql_enum]
            pub enum #order_by {
                #(#order_by_struk)*
            }
            impl OrderBy<Entity> for #order_by {
                fn config_default() -> Self {
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
        pub use #module::{
            Model as #sql,
            Entity as #model,
            Column as #column,
            ActiveModel as #active_model,
            ActiveModelAsyncImpl as #active_model_async_impl,
            #gql,
            #filter,
            #order_by,
        };
    };

    #[cfg(feature = "debug_macro")]
    debug_macro(&model_str, r.clone());

    r.into()
}
