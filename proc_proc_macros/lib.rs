use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(PartialEqString)]
pub fn derive_partial_eq_string(input: TokenStream) -> TokenStream {
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
