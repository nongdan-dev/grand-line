use heck::ToUpperCamelCase;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::Field;

pub fn push_order_by_field(
    f: &Field,
    fields: &mut Vec<TokenStream2>,
    matches: &mut Vec<TokenStream2>,
    direction_str: &str,
) {
    // sea_orm generated order_by_#direction(Column::Name)
    let column = format_ident!(
        "{}",
        f.ident.as_ref().unwrap().to_string().to_upper_camel_case(),
    );
    let direction_fn = format_ident!("order_by_{}", direction_str);
    // enum EnumField
    // graphql EnumField
    let name_str = format!("{}{}", column, direction_str.to_upper_camel_case());
    let name = format_ident!("{}", name_str);
    // push
    fields.push(quote! {
        #[graphql(name=#name_str)]
        #name,
    });
    matches.push(quote! {
        Self::#name => q.#direction_fn(Column::#column),
    });
}
