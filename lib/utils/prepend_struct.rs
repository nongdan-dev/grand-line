use crate::prelude::*;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{ToTokens, quote};
use syn::{ExprStruct, parse_macro_input};

pub fn prepend_struct(item: TokenStream, extra: TokenStream2) -> TokenStream {
    let input = parse_macro_input!(item as ExprStruct);
    let name = input.path.get_ident();
    let fields = input.fields.to_token_stream();
    let rest = parse_rest(&input, false);
    quote! {
        #name {
            #extra
            #fields
            #rest
        }
    }
    .into()
}
