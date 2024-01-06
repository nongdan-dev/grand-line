use std::str::FromStr;

use grand_line_macros::{field_quote, unwrap_enum, unwrap_enum_ref};
use heck::{ToLowerCamelCase, ToSnakeCase, ToUpperCamelCase};
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote, ToTokens};
use syn::{
    meta::parser, parse_macro_input, spanned::Spanned, DeriveInput, Expr, ExprStruct, Field,
    Fields, Ident, ItemFn, ItemStruct, Lit, LitStr, Type,
};

#[proc_macro_attribute]
pub fn model(_: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(item as ItemStruct);
    // ------------------------------------------------------------------------
    // get the original model name, and set the new name that sea_orm requires
    // get the original model name in snake case for sql table, non-plural
    let alias = input.ident;
    let alias_column = format_ident!("{}Column", alias);
    let alias_entity = format_ident!("{}Entity", alias);
    let alias_active_model = format_ident!("{}ActiveModel", alias);
    input.ident = Ident::new("Model", alias.span());
    let alias_str = alias.to_string();
    let alias_snake_str = alias_str.to_snake_case();
    let search_fn = format_ident!("search_{}", alias_snake_str);
    // ------------------------------------------------------------------------
    // insert built-in fields: id, created_at, updated_at
    let fields = unwrap_enum_ref!(input.fields => Fields::Named);
    let id = field_quote! {
        #[sea_orm(primary_key, column_type="Text", auto_increment=false)]
        pub id: String
    };
    let created_at = field_quote! {
        pub created_at: DateTime
    };
    let updated_at = field_quote! {
        pub updated_at: DateTime
    };
    fields.named.insert(0, id);
    fields.named.push(created_at);
    fields.named.push(updated_at);
    // ------------------------------------------------------------------------
    // filter / order_by fields
    let filter = format_ident!("{}Filter", alias);
    let order_by = format_ident!("{}OrderBy", alias);
    let mut filter_fields = vec![];
    let mut filter_matches = vec![];
    let mut order_by_fields = vec![];
    let mut order_by_matches = vec![];
    for ref f in unwrap_enum!(&input.fields => Fields::Named).named.iter() {
        push_filter_fields(f, &mut filter_fields, &mut filter_matches);
        push_order_by_fields(f, &mut order_by_fields, &mut order_by_matches);
    }
    push_filter_and_or(&filter, &mut filter_fields, &mut filter_matches, "and");
    push_filter_and_or(&filter, &mut filter_fields, &mut filter_matches, "or");
    push_filter_not(&filter, &mut filter_fields, &mut filter_matches);

    quote! {
        use sea_orm::*;
        use sea_orm::entity::prelude::*;
        #[derive(
            Clone,
            Debug,
            serde::Deserialize,
            serde::Serialize,
            async_graphql::SimpleObject,
            grand_line::internal::GrandLineModel,
            DeriveEntityModel,
        )]
        #[sea_orm(table_name=#alias_snake_str)]
        #[graphql(name=#alias_str)]
        #input

        //---------------------------------------------------------------------
        // empty relation and behavior to fullfil sea_orm model requirement
        // we would focus on the simplicity for now so we use those empty
        // if a complex query is required, use raw sql
        #[derive(Debug, EnumIter, DeriveRelation)]
        pub enum Relation {}
        impl ActiveModelBehavior for ActiveModel {
            fn new() -> Self {
                Self {
                    id: ActiveValue::Set(grand_line::internal::ulid()),
                    ..ActiveModelTrait::default()
                }
            }
        }

        //---------------------------------------------------------------------
        // filter
        #[derive(
          Clone,
          Debug,
          Default,
          serde::Deserialize,
          serde::Serialize,
          async_graphql::InputObject,
        )]
        pub struct #filter {
            #(#filter_fields)*
        }
        impl #filter {
            fn query(&self, mut q: Select<Entity>) -> Select<Entity> {
                q.filter(self.condition())
            }
            fn condition(&self) -> Condition {
                let this = self.clone();
                let mut c = Condition::all();
                #(#filter_matches)*
                c
            }
        }

        //---------------------------------------------------------------------
        // order_by
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
        impl #order_by {
            fn query(&self, q: Select<Entity>) -> Select<Entity> {
                match *self {
                    #(#order_by_matches)*
                }
            }
        }

        //---------------------------------------------------------------------
        // alias
        pub type #alias = Model;
        pub type #alias_column = Column;
        pub type #alias_entity = Entity;
        pub type #alias_active_model = ActiveModel;

        //---------------------------------------------------------------------
        // db runner
        pub async fn #search_fn(
            db: &DatabaseConnection,
            filter: Option<#filter>,
            order_by: Option<Vec<#order_by>>,
            page: Option<Pagination>,
        ) -> Result<Vec<#alias>, Box<dyn std::error::Error + Send + Sync>> {
            let mut q = Entity::find();
            if let Some(f) = filter {
                q = f.query(q);
            }
            if let Some(os) = order_by {
                if os.len() > 0 {
                    for o in os {
                        q = o.query(q);
                    }
                } else {
                    q = q.order_by_desc(Column::Id);
                }
            } else {
                q = q.order_by_desc(Column::Id);
            }
            let mut offset = 0;
            let mut limit = 10;
            if let Some(p) = page {
                if let Some(o) = p.offset {
                    offset = o;
                }
                if let Some(l) = p.limit {
                    limit = if l > 1000 { 1000 } else { l };
                }
            }
            Ok(q.offset(offset).limit(limit).all(db).await?)
        }
    }
    .into()
}

fn push_filter_fields(f: &Field, fields: &mut Vec<TokenStream2>, matches: &mut Vec<TokenStream2>) {
    push_filter_field(f, fields, matches, "eq");
    push_filter_field(f, fields, matches, "ne");
    let ty_str = unwrap_ty(&f.ty);
    if ty_str == "bool" {
        return;
    }
    push_filter_field(f, fields, matches, "is_in");
    push_filter_field(f, fields, matches, "is_not_in");
    let name = f.ident.clone().unwrap().to_string();
    if name == "id" || name.ends_with("_id") {
        return;
    }
    push_filter_field(f, fields, matches, "gt");
    push_filter_field(f, fields, matches, "gte");
    push_filter_field(f, fields, matches, "lt");
    push_filter_field(f, fields, matches, "lte");
    if ty_str != "String" {
        return;
    }
    push_filter_field(f, fields, matches, "like");
    push_filter_field(f, fields, matches, "not_like");
    push_filter_field(f, fields, matches, "starts_with");
    push_filter_field(f, fields, matches, "ends_with");
}

fn push_filter_field(
    f: &Field,
    fields: &mut Vec<TokenStream2>,
    matches: &mut Vec<TokenStream2>,
    op_str: &str,
) {
    // sea_orm generated Column::Name.op(v)
    let column = Ident::new(
        &f.ident.clone().unwrap().to_string().to_upper_camel_case(),
        f.span(),
    );
    let op = Ident::new(&op_str, f.span());
    // unwrap Option<type>
    // the type can be generic such as Box<type>
    let ty_str = unwrap_ty(&f.ty);
    let mut ty = TokenStream2::from_str(&ty_str).unwrap();
    // handle special operators
    let mut as_op_str = op_str.to_string();
    if op_str == "is_in" || op_str == "is_not_in" {
        as_op_str = op_str.replace("is_", "");
        ty = quote!(Vec<#ty>);
    }
    // struct struct_field_some_op
    // graphql structField_someOp
    let mut name_str = f.ident.clone().unwrap().to_string();
    let mut graphql_name_str = name_str.to_lower_camel_case();
    if op_str != "eq" {
        name_str = name_str + "_" + as_op_str.as_str();
        graphql_name_str = graphql_name_str + "_" + as_op_str.to_lower_camel_case().as_str();
    }
    let name = Ident::new(&name_str, f.span());
    // push
    fields.push(quote! {
            #[graphql(name=#graphql_name_str)]
            pub #name: Option<#ty>,
    });
    matches.push(quote! {
        if let Some(v) = this.#name {
            c = c.add(Column::#column.#op(v));
        }
    });
}

fn unwrap_ty(ty: &Type) -> String {
    let ty_str = ty.into_token_stream().to_string().replace(" ", "");
    if ty_str.starts_with("Box<") {
        return ty_str[4..(ty_str.len() - 1)].to_string();
    }
    if ty_str.starts_with("Option<") {
        return ty_str[7..(ty_str.len() - 1)].to_string();
    }
    ty_str
}

fn push_filter_and_or(
    f: &Ident,
    fields: &mut Vec<TokenStream2>,
    matches: &mut Vec<TokenStream2>,
    op_str: &str,
) {
    // sea_orm Condition::condition(v)
    let op = Ident::new(op_str, f.span());
    let graphql_name_str = op_str.to_uppercase();
    let condition_str = if op_str == "and" { "all" } else { "any" };
    let condition = Ident::new(condition_str, f.span());
    // push
    fields.push(quote! {
        #[graphql(name=#graphql_name_str)]
        pub #op: Option<Vec<#f>>,
    });
    matches.push(quote! {
        if let Some(v) = this.#op {
            let mut #op = Condition::#condition();
            for f in v {
                #op = #op.add(f.condition());
            }
            c = c.add(#op);
        }
    });
}
fn push_filter_not(f: &Ident, fields: &mut Vec<TokenStream2>, matches: &mut Vec<TokenStream2>) {
    // push
    fields.push(quote! {
        #[graphql(name="NOT")]
        pub not: Option<Box<#f>>,
    });
    matches.push(quote! {
        if let Some(v) = this.not {
            c = c.add(Condition::not(v.condition()));
        }
    });
}

fn push_order_by_fields(
    f: &Field,
    fields: &mut Vec<TokenStream2>,
    matches: &mut Vec<TokenStream2>,
) {
    push_order_by_field(f, fields, matches, "asc");
    push_order_by_field(f, fields, matches, "desc");
}
fn push_order_by_field(
    f: &Field,
    fields: &mut Vec<TokenStream2>,
    matches: &mut Vec<TokenStream2>,
    direction_str: &str,
) {
    // sea_orm generated order_by_#direction(Column::Name)
    let column = Ident::new(
        &f.ident.clone().unwrap().to_string().to_upper_camel_case(),
        f.span(),
    );
    let direction_fn = format_ident!("order_by_{}", direction_str);
    // enum EnumField
    // graphql EnumField
    let name_str = format!("{}{}", column, direction_str.to_upper_camel_case());
    let name = Ident::new(&name_str, f.span());
    // push
    fields.push(quote! {
        #[graphql(name=#name_str)]
        #name,
    });
    matches.push(quote! {
        Self::#name => q.#direction_fn(Column::#column),
    });
}

#[proc_macro_derive(
    GrandLineModel,
    attributes(
        // get sql table name and other sea orm config such as #[sea_orm(ignore)]
        sea_orm,
        // get original model name and other graphql config such as #[graphql(skip)]
        graphql,
        has_one,
        has_many,
        many_to_many,
        belongs_to,
    ),
)]
pub fn grand_line_model(item: TokenStream) -> TokenStream {
    let DeriveInput { attrs, .. } = parse_macro_input!(item as DeriveInput);
    let mut model_str = "".to_string();
    for a in attrs.iter() {
        if a.path().is_ident("graphql") {
            a.parse_nested_meta(|m| {
                model_str = m.value()?.parse::<LitStr>()?.value();
                Ok(())
            })
            .unwrap();
        } else if a.path().is_ident("has_one") {
            // TODO
        } else if a.path().is_ident("has_many") {
            // TODO
        } else if a.path().is_ident("many_to_many") {
            // TODO
        }
    }
    quote! {
        // TODO
    }
    .into()
}

#[proc_macro]
pub fn pagination(_: TokenStream) -> TokenStream {
    quote! {
        #[derive(
          Clone,
          Debug,
          Default,
          serde::Deserialize,
          serde::Serialize,
          async_graphql::InputObject,
        )]
        pub struct Pagination {
            pub limit: Option<u64>,
            pub offset: Option<u64>,
        }
    }
    .into()
}

#[proc_macro_attribute]
pub fn search(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut model = quote!();
    let mut field_str = "".to_string();
    let attr_parser = parser(|m| {
        if m.path.is_ident("name") {
            field_str = m.value()?.parse::<LitStr>()?.value();
        } else {
            if model.to_string() != "" {
                panic!("already specified model {}", model)
            }
            model = m.path.to_token_stream();
        }
        Ok(())
    });
    parse_macro_input!(attr with attr_parser);

    if model.to_string() == "" {
        panic!("missing model")
    }
    if field_str == "" {
        field_str = format!("search{}", model);
    }

    let input = parse_macro_input!(item as ItemFn);
    if input.sig.ident.to_string() != "handler" {
        panic!("expect handler, found {}", input.sig.ident.to_string())
    }

    let stmts = input.block.stmts;
    let search_fn = Ident::new(
        &format!("search_{}", model.to_string().to_snake_case()),
        input.sig.span(),
    );
    let model_query = Ident::new(
        &format!("{}Query", field_str.clone().to_upper_camel_case()),
        input.sig.span(),
    );
    let model_filter = Ident::new(&format!("{}Filter", model), input.sig.span());
    let model_order_by = Ident::new(&format!("{}OrderBy", model), input.sig.span());
    let field = Ident::new(&field_str.clone().to_snake_case(), input.sig.span());

    quote! {
        use sea_orm::*;
        use sea_orm::entity::prelude::*;
        #[derive(Default)]
        pub struct #model_query;
        #[async_graphql::Object]
        impl #model_query {
            #[graphql(name=#field_str)]
            async fn #field(
                &self,
                ctx: &async_graphql::Context<'_>,
                filter: Option<#model_filter>,
                order_by: Option<Vec<#model_order_by>>,
                page: Option<Pagination>,
            ) -> Result<Vec<#model>, Box<dyn std::error::Error + Send + Sync>> {
                let next = |
                    extra_filter: Option<#model_filter>,
                    default_order_by: Option<Vec<#model_order_by>>,
                | {
                    let f = if let Some(f1) = filter {
                        if let Some(f2) = extra_filter {
                            Some(#model_filter {
                                and: Some(vec![f1, f2]),
                                ..Default::default()
                            })
                        } else {
                            Some(f1)
                        }
                    } else if let Some(f2) = extra_filter {
                        Some(f2)
                    } else {
                        None
                    };
                    let o = if let Some(o1) = order_by {
                        if o1.len() > 0 {
                            Some(o1)
                        } else {
                            default_order_by
                        }
                    } else {
                        default_order_by
                    };
                    let db = ctx.data_unchecked::<DatabaseConnection>();
                    #search_fn(db, f, o, page)
                };
                #(#stmts)*
            }
        }
    }
    .into()
}

#[proc_macro]
pub fn filter(item: TokenStream) -> TokenStream {
    default_struct(item, "Filter", "Some", "Some")
}

#[proc_macro]
pub fn active_model(item: TokenStream) -> TokenStream {
    default_struct(item, "ActiveModel", "sea_orm::ActiveValue::Set", "")
}

fn default_struct(
    item: TokenStream,
    suffix: &str,
    field_wrap: &str,
    return_wrap: &str,
) -> TokenStream {
    let fw = TokenStream2::from_str(&field_wrap).unwrap();
    let input = parse_macro_input!(item as ExprStruct);
    let mut fields = vec![];
    for f in input.fields.into_iter() {
        let m = f.member;
        let e = f.expr;
        if let Expr::Lit(l) = e.clone() {
            if let Lit::Str(s) = l.lit {
                let v = s.value();
                fields.push(quote!(#m:#fw(#v.to_string()),));
            } else {
                fields.push(quote!(#m:#fw(#l),));
            }
        } else {
            fields.push(quote!(#m:#fw(#e),));
        }
    }
    let name = Ident::new(
        &format!("{}{}", input.path.get_ident().to_token_stream(), suffix),
        input.path.span(),
    );
    let mut r = quote! {
        #name {
            #(#fields)*
            ..Default::default()
        }
    };
    if return_wrap != "" {
        let rw = TokenStream2::from_str(&return_wrap).unwrap();
        r = quote!(#rw(#r))
    }
    r.into()
}
