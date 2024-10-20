use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::Ident;

pub fn push_filter_not(f: &Ident, fields: &mut Vec<TokenStream2>, matches: &mut Vec<TokenStream2>) {
    // push
    fields.push(quote! {
        #[graphql(name="NOT")]
        pub not: Option<Box<#f>>,
    });
    matches.push(quote! {
        if let Some(v) = this.not {
            c = c.add(Condition::not(v.condition()));
        }
    });
}
