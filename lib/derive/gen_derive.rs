use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

pub fn gen_derive(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    for a in input.attrs.iter() {
        if a.path().is_ident("has_one") {
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
