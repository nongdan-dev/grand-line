use crate::prelude::*;
use syn::ExprStruct;

pub fn parse_rest(input: &ExprStruct, default: bool) -> TokenStream2 {
    let rest = input.rest.to_token_stream();
    if str!(rest).trim() == "" {
        if default {
            ts2!("..Default::default()")
        } else {
            ts2!("")
        }
    } else {
        quote!(..#rest)
    }
}
