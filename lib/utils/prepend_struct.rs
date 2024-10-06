use crate::prelude::*;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, ExprStruct};

pub fn prepend_struct(item: TokenStream, extra: TokenStream2, update: bool) -> TokenStream {
    let input = parse_macro_input!(item as ExprStruct);
    let name = input.path.get_ident();
    let fields = input.fields.to_token_stream();
    let rest = parse_rest(&input, false);
    if update {
        let rest = rest.to_string();
        if rest == "Default::default()" {
            panic!("active_update should not have rest with Default::default()");
        }
    }
    quote! {
        #name {
            #extra
            #fields
            #rest
        }
    }
    .into()
}
