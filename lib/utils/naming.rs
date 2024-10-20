use heck::{ToLowerCamelCase, ToSnakeCase, ToUpperCamelCase};
use quote::format_ident;
use std::fmt::Display;
use syn::Ident;

// ============================================================================
// utility macros to be generated

pub fn util_macros() -> Vec<&'static str> {
    vec![
        "filter",
        "filter_some",
        "order_by",
        "order_by_some",
        "active_model",
        "active_create",
        "active_update",
    ]
}
pub fn crud_util_macros() -> Vec<&'static str> {
    vec!["create", "search", "count", "detail", "update", "delete"]
}
pub fn util_macro(pre: impl Display, suf: impl Display) -> Ident {
    let pre = pre.to_string().to_snake_case();
    let suf = suf.to_string().to_snake_case();
    format_ident!("{}_{}", pre, suf)
}

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

pub fn ty_model(model: impl Display) -> Ident {
    ty(model, "")
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

// ============================================================================
// rust db fn builtin for crud snake_case

fn db_fn(pre: impl Display, suf: impl Display) -> Ident {
    let pre = pre.to_string().to_snake_case();
    let suf = suf.to_string().to_snake_case();
    format_ident!("{}_db_{}", pre, suf)
}

pub fn db_search(model: impl Display) -> Ident {
    db_fn(model, "search")
}
pub fn db_count(model: impl Display) -> Ident {
    db_fn(model, "count")
}
pub fn db_detail(model: impl Display) -> Ident {
    db_fn(model, "detail")
}
pub fn db_delete(model: impl Display) -> Ident {
    db_fn(model, "delete")
}
