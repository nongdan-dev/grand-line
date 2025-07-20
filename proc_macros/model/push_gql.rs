use crate::prelude::*;
use syn::Field;

pub fn push_gql(
    f: &Field,
    struk: &mut Vec<TokenStream2>,
    resolver: &mut Vec<TokenStream2>,
    look_ahead: &mut Vec<TokenStream2>,
    into: &mut Vec<TokenStream2>,
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
    look_ahead.push(quote! {
        if l.field(#gql_name).exists() {
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
