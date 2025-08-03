use crate::prelude::*;
use syn::{DeriveInput, parse_macro_input};

pub fn gen_partial_eq_string(input: TokenStream) -> TokenStream {
    let ty = parse_macro_input!(input as DeriveInput).ident;
    quote! {
        impl PartialEq<&str> for #ty {
            fn eq(&self, other: &&str) -> bool {
                self.to_string() == *other
            }
        }
        impl PartialEq<String> for #ty {
            fn eq(&self, other: &String) -> bool {
                self.to_string() == *other
            }
        }
        impl PartialEq<#ty> for &str {
            fn eq(&self, other: &#ty) -> bool {
                *self == other.to_string()
            }
        }
        impl PartialEq<#ty> for String {
            fn eq(&self, other: &#ty) -> bool {
                *self == other.to_string()
            }
        }
    }
    .into()
}
