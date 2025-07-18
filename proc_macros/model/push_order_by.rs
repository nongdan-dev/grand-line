use crate::prelude::*;
use heck::ToUpperCamelCase;
use proc_macro2::TokenStream as TokenStream2;
use quote::{ToTokens, quote};
use syn::Field;

pub fn push_order_by(f: &Field, struk: &mut Vec<TokenStream2>, query: &mut Vec<TokenStream2>) {
    push(f, struk, query, "asc");
    push(f, struk, query, "desc");
}
fn push(
    f: &Field,
    struk: &mut Vec<TokenStream2>,
    query: &mut Vec<TokenStream2>,
    direction_str: &str,
) {
    // sea_orm generated order_by_#direction(Column::Name)
    let column = f
        .ident
        .to_token_stream()
        .to_string()
        .to_upper_camel_case()
        .parse::<TokenStream2>()
        .unwrap();
    let direction_fn = format!("order_by_{}", direction_str)
        .parse::<TokenStream2>()
        .unwrap();
    // enum EnumField
    // graphql EnumField
    let name = pascal!(&column, direction_str);
    let gql_name = name.to_string();
    // push
    struk.push(quote! {
        #[graphql(name=#gql_name)]
        #name,
    });
    query.push(quote! {
        Self::#name => q.#direction_fn(Column::#column),
    });
}
