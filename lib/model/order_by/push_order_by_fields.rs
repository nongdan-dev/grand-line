use crate::prelude::*;
use proc_macro2::TokenStream as TokenStream2;
use syn::Field;

pub fn push_order_by_fields(
    f: &Field,
    fields: &mut Vec<TokenStream2>,
    matches: &mut Vec<TokenStream2>,
) {
    push_order_by_field(f, fields, matches, "asc");
    push_order_by_field(f, fields, matches, "desc");
}
