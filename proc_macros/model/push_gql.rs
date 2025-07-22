use std::collections::HashSet;

use crate::prelude::*;
use syn::Field;

pub fn push_gql(
    f: &Field,
    dep_sql: &Vec<String>,
    dep_gql: &Vec<String>,
    struk: &mut Vec<TokenStream2>,
    resolver: &mut Vec<TokenStream2>,
    into: &mut Vec<TokenStream2>,
    columns: &mut Vec<TokenStream2>,
) {
    let name = f.ident.to_token_stream();
    let gql_name = camel_str!(name.to_token_stream());
    let ty = &f.ty;
    let (opt, uw) = unwrap_option(ty.to_token_stream());

    struk.push(quote! {
        #name: Option<#uw>,
    });

    let res = if opt {
        quote!(v)
    } else {
        quote!(v.unwrap_or_default())
    };
    resolver.push(quote! {
        #[graphql(name=#gql_name)]
        async fn #name(&self) -> #ty {
            let v = self.#name.clone();
            #res
        }
    });

    into.push(if opt {
        quote! {
            #name: v.#name,
        }
    } else {
        quote! {
            #name: Some(v.#name),
        }
    });

    let sql = dep_sql
        .iter()
        .enumerate()
        .filter(|(_, v)| **v == str!(name))
        .map(|(i, _)| i)
        .collect::<Vec<_>>();
    let mut gql = dep_gql
        .iter()
        .enumerate()
        .filter(|(i, _)| sql.contains(i))
        .map(|(_, v)| camel_str!(v))
        .collect::<Vec<_>>();
    gql.push(gql_name);
    gql = gql
        .iter()
        .collect::<HashSet<_>>()
        .into_iter()
        .map(|f| f.to_string())
        .collect::<Vec<_>>();
    let column = pascal!(name.to_token_stream());
    columns.push(quote! {
        #(m.insert(#gql, Column::#column);)*
    });
}
