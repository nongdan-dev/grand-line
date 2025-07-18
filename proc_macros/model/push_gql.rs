use crate::prelude::*;
use proc_macro2::TokenStream as TokenStream2;
use quote::{ToTokens, quote};
use syn::Field;

pub fn push_gql(
    f: &Field,
    struk: &mut Vec<TokenStream2>,
    resolver: &mut Vec<TokenStream2>,
    look_ahead: &mut Vec<TokenStream2>,
    into: &mut Vec<TokenStream2>,
) {
    let name = f.ident.clone();
    let gql_name = camel_str(name.to_token_stream(), "");
    let ty = &f.ty;
    let (opt, ty_str) = unwrap_option(ty.to_token_stream());

    let ty_unwrapped = ty_str.parse::<TokenStream2>().unwrap();
    struk.push(quote! {
        #name: Option<#ty_unwrapped>,
    });

    let res = if opt {
        quote! {
            self.#name
        }
    } else {
        quote! {
            self.#name.clone().unwrap_or_default()
        }
    };
    resolver.push(quote! {
        #[graphql(name=#gql_name)]
        async fn #name(&self) -> #ty {
            #res
        }
    });

    let ty = pascal!(name.to_token_stream(), "");
    look_ahead.push(quote! {
        if l.field(#gql_name).exists() {
            q = q.column(Column::#ty)
        }
    });

    let res = if opt {
        quote! {
            #name: v.#name,
        }
    } else {
        quote! {
            #name: Some(v.#name),
        }
    };
    into.push(quote! {
        #res
    });
}
