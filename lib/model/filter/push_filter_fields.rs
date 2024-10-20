use crate::prelude::*;
use proc_macro2::TokenStream as TokenStream2;
use quote::ToTokens;
use syn::Field;

pub fn push_filter_fields(
    f: &Field,
    fields: &mut Vec<TokenStream2>,
    matches: &mut Vec<TokenStream2>,
) {
    push_filter_field(f, fields, matches, "eq");
    push_filter_field(f, fields, matches, "ne");
    let ty_str = unwrap_ty(f.ty.to_token_stream());
    if ty_str == "bool" {
        return;
    }
    push_filter_field(f, fields, matches, "is_in");
    push_filter_field(f, fields, matches, "is_not_in");
    let name = f.ident.as_ref().unwrap().to_string();
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
