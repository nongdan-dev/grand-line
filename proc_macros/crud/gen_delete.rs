use crate::prelude::*;
use proc_macro::TokenStream;
use quote::quote;

pub fn gen_delete(attr: TokenStream, item: TokenStream) -> TokenStream {
    let a = parse_attr!(attr);
    let g = parse_resolver!(ty_mutation, item, gql_delete(&a.model));
    let (a, mut g) = check_crud_io(a, g);

    let output = ty_gql(&a.model);
    let db_fn = rs_gql_delete(&a.model);

    if !a.resolver_inputs {
        g.inputs = quote! {
            id: String,
        };
    }

    if !a.resolver_output {
        g.output = quote! {
            Option<#output>
        };

        let body = g.body;
        g.body = quote! {
            #body
            #db_fn(ctx, &tx, &id).await?
        };
    }

    g.no_tx = a.no_tx;
    gen_resolver(g)
}
