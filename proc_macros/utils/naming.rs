use heck::{ToLowerCamelCase, ToSnakeCase, ToUpperCamelCase};
use proc_macro2::TokenStream as TokenStream2;
use std::fmt::Display;

// ============================================================================
// PascalCase: same for graphql types and rust types

fn pascal(pre: impl Display, suf: impl Display) -> TokenStream2 {
    pascal_str(pre, suf).parse().unwrap()
}
pub fn pascal_str(pre: impl Display, suf: impl Display) -> String {
    let pre = pre.to_string().to_upper_camel_case();
    let suf = suf.to_string().to_upper_camel_case();
    format!("{}{}", pre, suf)
}

pub fn ty_query(pre: impl Display) -> TokenStream2 {
    pascal(pre, "Query")
}
pub fn ty_mutation(pre: impl Display) -> TokenStream2 {
    pascal(pre, "Mutation")
}

pub fn ty_sql(model: impl Display) -> TokenStream2 {
    pascal(model, "Sql")
}
pub fn ty_gql(model: impl Display) -> TokenStream2 {
    pascal(model, "Gql")
}
pub fn ty_column(model: impl Display) -> TokenStream2 {
    pascal(model, "Column")
}
pub fn ty_active_model(model: impl Display) -> TokenStream2 {
    pascal(model, "ActiveModel")
}
pub fn ty_filter(model: impl Display) -> TokenStream2 {
    pascal(model, "Filter")
}
pub fn ty_filter_combine(model: impl Display) -> TokenStream2 {
    pascal(model, "FilterCombine")
}
pub fn ty_order_by(model: impl Display) -> TokenStream2 {
    pascal(model, "OrderBy")
}
pub fn ty_order_by_combine(model: impl Display) -> TokenStream2 {
    pascal(model, "OrderByCombine")
}

pub fn ty_input(name: impl Display) -> TokenStream2 {
    pascal(name, "")
}

// ============================================================================
// camelCase: graphql fields, always string

pub fn camel_str(pre: impl Display, suf: impl Display) -> String {
    let pre = pre.to_string().to_lower_camel_case();
    let suf = suf.to_string().to_upper_camel_case();
    format!("{}{}", pre, suf)
}

pub fn gql_create(model: impl Display) -> String {
    camel_str(model, "Create")
}
pub fn gql_search(model: impl Display) -> String {
    camel_str(model, "Search")
}
pub fn gql_count(model: impl Display) -> String {
    camel_str(model, "Count")
}
pub fn gql_detail(model: impl Display) -> String {
    camel_str(model, "Detail")
}
pub fn gql_update(model: impl Display) -> String {
    camel_str(model, "Update")
}
pub fn gql_delete(model: impl Display) -> String {
    camel_str(model, "Delete")
}

// ============================================================================
// rs: static methods

fn rs_static(struk: impl Display, method: impl Display) -> TokenStream2 {
    let pre = struk.to_string().to_upper_camel_case();
    let suf = method.to_string().to_snake_case();
    format!("{}::{}", pre, suf).parse().unwrap()
}

pub fn rs_gql_search(model: impl Display) -> TokenStream2 {
    rs_static(model, "gql_search")
}
pub fn rs_gql_count(model: impl Display) -> TokenStream2 {
    rs_static(model, "gql_count")
}
pub fn rs_gql_detail(model: impl Display) -> TokenStream2 {
    rs_static(model, "gql_detail")
}
pub fn rs_gql_delete(model: impl Display) -> TokenStream2 {
    rs_static(model, "gql_delete")
}
