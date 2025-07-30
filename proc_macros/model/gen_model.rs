use crate::prelude::*;
use syn::{Fields, ItemStruct, parse_macro_input};

pub fn gen_model(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr = parse_macro_input!(attr as AttrParse);
    let mut struk = parse_macro_input!(item as ItemStruct);
    let ModelAttr {
        no_created_at,
        no_updated_at,
        no_deleted_at,
        no_by_id,
        limit_default,
        limit_max,
    } = attr.into_inner::<ModelAttr>("model");
    // ------------------------------------------------------------------------
    // insert built-in fields: id, created_at, updated_at...
    let model_str = str!(struk.ident);
    let mut fields = match struk.fields {
        Fields::Named(f) => f.named.into_iter().collect::<Vec<_>>(),
        _ => panic!("{} struct fields must be Fields::Named", model_str),
    };
    fields.insert(
        0,
        field! {
            #[sea_orm(primary_key, column_type="String(StringLen::N(26))", auto_increment=false)]
            pub id: String
        },
    );
    if !no_created_at {
        fields.push(field! {
            pub created_at: DateTimeUtc
        });
        if !no_by_id {
            fields.push(field! {
                pub created_by_id: Option<String>
            });
        }
    }
    if !no_updated_at {
        fields.push(field! {
            pub updated_at: Option<DateTimeUtc>
        });
        if !no_by_id {
            fields.push(field! {
                pub updated_by_id: Option<String>
            });
        }
    }
    let mut config_col_deleted_at = quote!(None);
    if !no_deleted_at {
        fields.push(field! {
            pub deleted_at: Option<DateTimeUtc>
        });
        if !no_by_id {
            fields.push(field! {
                pub deleted_by_id: Option<String>
            });
        }
        config_col_deleted_at = quote!(Some(Column::DeletedAt))
    }
    // ------------------------------------------------------------------------
    // parse macro attributes, extract and validate
    let (vfields, exprs, gfields, fields) = extract_and_validate_fields(&model_str, &fields);
    // ------------------------------------------------------------------------
    // get the original model name, and set the new name that sea_orm requires
    // get the original model name in snake case for sql table, non-plural
    let model = ts2!(model_str);
    struk.ident = format_ident!("Model");
    struk.fields = Fields::Named(fields);
    let module = snake!(model);
    let sql = ty_sql(&model);
    let gql = ty_gql(&model);
    let column = ty_column(&model);
    let active_model = ty_active_model(&model);
    let gql_alias = str!(model);
    let sql_alias = snake_str!(model);
    // ------------------------------------------------------------------------
    // generate virtual resolvers
    let mut virtuals = Vec::<Box<dyn GenVirtual>>::new();
    for attrs in vfields {
        let map = attrs
            .clone()
            .into_iter()
            .map(|a| (a.attr.clone(), a))
            .collect::<HashMap<_, _>>();
        let relation = RelationTy::all()
            .iter()
            .find(|r| map.contains_key(&r.to_string()))
            .map(|r| r.to_owned());
        if let Some(ty) = relation {
            let a = map.get(&ty).unwrap().clone();
            virtuals.push(Box::new(GenRelation {
                model: model_str.clone(),
                ty: ty.parse().unwrap(),
                a: RelationAttr::new(a),
            }));
        }
        // TODO: other kinds of virtual
    }
    // ------------------------------------------------------------------------
    // filter / order_by fields
    let filter = ty_filter(&model);
    let order_by = ty_order_by(&model);
    let (mut filter_struk, mut filter_query) = (vec![], vec![]);
    let (mut order_by_struk, mut order_by_query) = (vec![], vec![]);
    for (f, _) in &gfields {
        push_filter(f, &mut filter_struk, &mut filter_query);
        push_order_by(f, &mut order_by_struk, &mut order_by_query);
    }
    push_filter_and_or_not(&filter, &mut filter_struk, &mut filter_query);

    // ------------------------------------------------------------------------
    // gql fields
    let (mut gql_struk, mut gql_resolver, gql_into, gql_select) = gql_fields(&gfields, &virtuals);
    let (gql_struk2, gql_resolver2, gql_select_as) = gql_exprs(&exprs);
    gql_struk.extend(gql_struk2);
    gql_resolver.extend(gql_resolver2);
    for f in virtuals.iter() {
        gql_resolver.push(f.gen_resolver_fn());
    }
    // ------------------------------------------------------------------------
    // active model utils
    let am_id = quote! {
        if !matches!(am.id, Set(_)) {
            am.id = Set(ulid::Ulid::new().to_string());
        }
    };
    let am_created_at = if no_created_at {
        ts2!()
    } else {
        quote! {
            if !matches!(am.created_at, ActiveValue::Set(_)) {
                am.created_at = ActiveValue::Set(chrono::Utc::now());
            }
        }
    };
    let am_updated_at = if no_updated_at {
        ts2!()
    } else {
        quote! {
            if !matches!(am.updated_at, ActiveValue::Set(_)) {
                am.updated_at = ActiveValue::Set(Some(chrono::Utc::now()));
            }
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
                GrandLineModel,
            )]
            #[sea_orm(table_name=#sql_alias)]
            #struk

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

            type GqlSelect = LazyLock<HashMap<&'static str, Column>>;
            static GQL_SELECT: GqlSelect = LazyLock::new(|| {
                let mut m = HashMap::new();
                #(#gql_select)*
                m
            });
            type GqlSelectAs = LazyLock<HashMap<&'static str, (&'static str, sea_query::SimpleExpr)>>;
            static GQL_SELECT_AS: GqlSelectAs = LazyLock::new(|| {
                let mut m = HashMap::new();
                #(#gql_select_as)*
                m
            });
            impl EntityX<Model, ActiveModel, #filter, #order_by, #gql> for Entity {
                fn config_active_create(mut am: ActiveModel) -> ActiveModel {
                    #am_id
                    #am_created_at
                    am
                }
                fn config_active_update(mut am: ActiveModel) -> ActiveModel {
                    #am_updated_at
                    am
                }
                fn config_col_id() -> Column {
                    Column::Id
                }
                fn config_col_deleted_at() -> Option<Column> {
                    #config_col_deleted_at
                }
                fn config_gql_select(field: &str) -> (Option<Self::Column>, Option<(String, sea_query::SimpleExpr)>) {
                    let o1 = GQL_SELECT.get(field);
                    if o1.is_some() {
                        return (o1.copied(), None);
                    }
                    let o2 = GQL_SELECT_AS.get(field).map(|s| (s.0.to_string(), s.1.clone()));
                    (None, o2)
                }
                fn config_limit() -> ConfigLimit {
                    ConfigLimit {
                        default: #limit_default,
                        max: #limit_max,
                    }
                }
            }

            #[input]
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
                    todo!("TODO:")
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

            #[enunn]
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
            #gql,
            #filter,
            #order_by,
        };
    };

    #[cfg(feature = "debug_macro")]
    debug_macro(&model_str, r.clone());

    r.into()
}
