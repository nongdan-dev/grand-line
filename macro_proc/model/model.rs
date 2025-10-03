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
    if !a.no_deleted_at {
        fields.push(field!(pub deleted_at: Option<DateTimeUtc>));
        if !a.no_by_id {
            fields.push(field!(pub deleted_by_id: Option<String>));
        }
    }

    // ------------------------------------------------------------------------
    // parse macro attributes, extract and validate fields
    let model_str = s!(model);
    let (defs, virs, exprs, gfields, fields) = attr_parse(&model_str, &fields);

    // ------------------------------------------------------------------------
    // get the original model name, and set the new name that sea_orm requires
    // get the original model name in snake case for sql table, non-plural
    item.ident = ident!("Model");
    item.fields = Fields::Named(fields);
    let module = snake!(model);
    let sql = ty_sql(&model);
    let gql = ty_gql(&model);
    let column = ty_column(&model);
    let active_model = ty_active_model(&model);
    let gql_alias = s!(model);
    let sql_alias = snake_str!(model);

    // ------------------------------------------------------------------------
    // entity limit_config
    let (limit_default, limit_max) = (a.limit_default, a.limit_max);
    let limit_config = quote! {
        LimitConfig {
            default: #limit_default,
            max: #limit_max,
        }
    };

    // ------------------------------------------------------------------------
    // active model default
    let mut am_defs = vec![];
    let mut self_am_defs = vec![];
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
        self_am_defs.push(quote! {
            if !matches!(self.#name, Set(_)) {
                self.#name = Set(#raw);
            }
        });
    }

    // ------------------------------------------------------------------------
    // active model get/set
    let am_get_created_at = if a.no_created_at {
        quote!(NotSet)
    } else {
        quote!(self.created_at.clone())
    };
    let am_set_created_at = if a.no_created_at {
        quote!(self)
    } else {
        quote! {
            self.created_at = Set(v);
            self
        }
    };
    let am_get_updated_at = if a.no_updated_at {
        quote!(NotSet)
    } else {
        quote!(self.updated_at.clone())
    };
    let am_set_updated_at = if a.no_updated_at {
        quote!(self)
    } else {
        quote! {
            self.updated_at = Set(Some(v));
            self
        }
    };
    let am_get_deleted_at = if a.no_deleted_at {
        quote!(NotSet)
    } else {
        quote!(self.deleted_at.clone())
    };
    let am_set_deleted_at = if a.no_deleted_at {
        quote!(self)
    } else {
        quote! {
            self.deleted_at = Set(Some(v));
            self
        }
    };

    // ------------------------------------------------------------------------
    // filter / order_by
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
    // filter has_deleted_at
    let has_deleted_at = if a.no_deleted_at {
        quote!(false)
    } else {
        quote! {
            !self.deleted_at.is_undefined() ||
            !self.deleted_at_ne.is_undefined() ||
            self.deleted_at_in.is_some() ||
            self.deleted_at_not_in.is_some() ||
            self.deleted_at_gt.is_some() ||
            self.deleted_at_gte.is_some() ||
            self.deleted_at_lt.is_some() ||
            self.deleted_at_lte.is_some()
        }
    };

    // ------------------------------------------------------------------------
    // virtual resolvers
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
                    ra: a.clone().into_with_validate(),
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
    // gql
    let (mut gql_struk, mut gql_resolver, gql_into, sql_cols) = gql_fields(&gfields);
    let mut gql_select = gql_virtuals(&vgens);
    let (gql_struk2, gql_resolver2, gql_select2, sql_exprs) = gql_exprs(&exprs);
    gql_struk.extend(gql_struk2);
    gql_resolver.extend(gql_resolver2);
    gql_select.extend(gql_select2);
    for f in vgens {
        gql_resolver.push(f.resolver_fn());
    }

    let r = quote! {
        mod #module {
            use super::*;
            pub use sea_orm::{entity::prelude::*, prelude::*, *};

            #[derive(
                Debug,
                Clone,
                Default,
                DeriveEntityModel,
            )]
            #[sea_orm(table_name=#sql_alias)]
            #item

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

            impl ActiveModelBehavior for ActiveModel {
                // no support for ActiveModelBehavior
                // instead use the following macros: default, am_create, am_update, am_delete
            }
            #[derive(Debug, EnumIter, DeriveRelation)]
            pub enum Relation {
                // TODO:
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

            impl EntityX for Entity {
                type M = Model;
                type A = ActiveModel;
                type F = #filter;
                type O = #order_by;
                type G = #gql;
                fn _model_name() -> &'static str {
                    #model_str
                }
                fn _limit_config() -> LimitConfig {
                    #limit_config
                }
                fn _sql_cols() -> &'static LazyLock<HashMap<&'static str, Self::Column>> {
                    &SQL_COLS
                }
                fn _sql_exprs() -> &'static LazyLock<HashMap<&'static str, sea_query::SimpleExpr>> {
                    &SQL_EXPRS
                }
                fn _gql_select() -> &'static LazyLock<HashMap<&'static str, Vec<&'static str>>> {
                    &GQL_SELECT
                }
            }

            impl ModelX<Entity> for Model {
                fn _get_id(&self) -> String {
                    self.id.clone()
                }
                fn _to_gql(self) -> #gql {
                    #gql {
                        #(#gql_into)*
                        ..Default::default()
                    }
                }
            }

            impl ActiveModelX<Entity> for ActiveModel {
                fn _set_default_values(mut self) -> Self {
                    #(#self_am_defs)*
                    self
                }
                fn _get_id(&self) -> ActiveValue<String> {
                    self.id.clone()
                }
                fn _set_id(mut self, v: &str) -> Self {
                    self.id = Set(v.to_string());
                    self
                }
                fn _get_created_at(&self) -> ActiveValue<DateTimeUtc> {
                    #am_get_created_at
                }
                fn _set_created_at(mut self, v: DateTimeUtc) -> Self {
                    #am_set_created_at
                }
                fn _get_updated_at(&self) -> ActiveValue<Option<DateTimeUtc>> {
                    #am_get_updated_at
                }
                fn _set_updated_at(mut self, v: DateTimeUtc) -> Self {
                    #am_set_updated_at
                }
                fn _get_deleted_at(&self) -> ActiveValue<Option<DateTimeUtc>> {
                    #am_get_deleted_at
                }
                fn _set_deleted_at(mut self, v: DateTimeUtc) -> Self {
                    #am_set_deleted_at
                }
            }

            impl GqlModel<Entity> for #gql {
                fn _set_id(mut self, v: &str) -> Self {
                    self.id = Some(v.to_string());
                    self
                }
            }

            #[gql_input]
            pub struct #filter {
                #(#filter_struk)*
            }
            impl Filter<Entity> for #filter {
                fn _combine_and(a: Self, b: Self) -> Self {
                    Self {
                        and: Some(vec![a, b]),
                        ..Default::default()
                    }
                }
                fn _has_deleted_at(&self) -> bool {
                    #has_deleted_at
                }
                fn _get_and(&self) -> Option<Vec<Self>> {
                    self.and.clone()
                }
                fn _get_or(&self) -> Option<Vec<Self>> {
                    self.or.clone()
                }
                fn _get_not(&self) -> Option<Self> {
                    self.not.clone().map(|b| *b)
                }
            }
            impl IntoCondition for #filter {
                fn into_condition(&self) -> Condition {
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
                fn conf_default() -> Self {
                    Self::IdDesc
                }
            }
            impl ChainSelect<Entity> for #order_by {
                fn chain_select(&self, q: Select<Entity>) -> Select<Entity> {
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
            #gql,
            #filter,
            #order_by,
        };
    };

    #[cfg(feature = "debug_macro")]
    debug_macro(&model_str, r.clone());

    r.into()
}
