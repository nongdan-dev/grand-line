use proc_macro2::TokenStream as TokenStream2;
use quote::{ToTokens, quote};
use syn::ExprStruct;

pub fn parse_rest(input: &ExprStruct, default: bool) -> TokenStream2 {
    let rest = input.rest.to_token_stream();
    let binding = rest.to_string();
    let rest_str = binding.trim();
    if rest_str == "" {
        if default { "..Default::default()" } else { "" }
            .to_owned()
            .parse()
            .unwrap()
    } else {
        quote! {
            ..#rest
        }
    }
}
