use crate::prelude::*;
use syn::Field;

pub fn push_gql(
    f: &Field,
    struk: &mut Vec<TokenStream2>,
    resolver: &mut Vec<TokenStream2>,
    look_ahead: &mut Vec<TokenStream2>,
    into: &mut Vec<TokenStream2>,
    sql_deps: &Vec<String>,
    dep_fields: &Vec<String>,
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

    let column = pascal!(name.to_token_stream());
    let ii = sql_deps
        .iter()
        .enumerate()
        .filter(|(_, v)| **v == str!(name))
        .map(|(i, _)| i)
        .collect::<Vec<usize>>();
    let x = dep_fields
        .iter()
        .enumerate()
        .filter(|(i, _)| ii.contains(i))
        .map(|(_, v)| camel_str!(v))
        .map(|v| quote!(|| l.field(#v).exists()))
        .collect::<Vec<TokenStream2>>();
    // TODO: make x unique arr
    look_ahead.push(quote! {
        if l.field(#gql_name).exists() #(#x)* {
            q = q.column(Column::#column)
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
}
