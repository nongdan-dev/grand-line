use heck::{ToLowerCamelCase, ToUpperCamelCase};
use quote::format_ident;
use std::fmt::Display;
use syn::Ident;

// ============================================================================
// graphql type UpperCamelCase

fn ty(pre: impl Display, suf: impl Display) -> Ident {
    let pre = pre.to_string().to_upper_camel_case();
    let suf = suf.to_string().to_upper_camel_case();
    format_ident!("{}{}", pre, suf)
}

pub fn ty_query(query: impl Display) -> Ident {
    ty(query, "query")
}
pub fn ty_mutation(mutation: impl Display) -> Ident {
    ty(mutation, "mutation")
}

pub fn ty_output(model: impl Display) -> Ident {
    ty(model, "gql")
}
pub fn ty_filter(model: impl Display) -> Ident {
    ty(model, "filter")
}
pub fn ty_order_by(model: impl Display) -> Ident {
    ty(model, "order_by")
}

pub fn ty_input(name: impl Display) -> Ident {
    ty(name, "")
}

// ============================================================================
// graphql field name builtin for crud lowerCamelCase

fn name(pre: impl Display, suf: impl Display) -> String {
    let pre = pre.to_string().to_lower_camel_case();
    let suf = suf.to_string().to_upper_camel_case();
    format!("{}{}", pre, suf)
}
pub fn name_create(model: impl Display) -> String {
    name(model, "create")
}
pub fn name_search(model: impl Display) -> String {
    name(model, "search")
}
pub fn name_count(model: impl Display) -> String {
    name(model, "count")
}
pub fn name_detail(model: impl Display) -> String {
    name(model, "detail")
}
pub fn name_update(model: impl Display) -> String {
    name(model, "update")
}
pub fn name_delete(model: impl Display) -> String {
    name(model, "delete")
}
