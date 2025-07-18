use crate::prelude::*;
use proc_macro::TokenStream;
use quote::quote;

pub fn gen_count(attr: TokenStream, item: TokenStream) -> TokenStream {
    let a = parse_attr!(attr);
    let g = parse_resolver!(ty_query, item, gql_count(&a.model));
    let (a, mut g) = check_crud_io(a, g);

    let model_filter = ty_filter(&a.model);
    let db_fn = rs_gql_count(&a.model);

    if !a.resolver_inputs {
        g.inputs = quote! {
            filter: Option<#model_filter>,
        };
    }

    if !a.resolver_output {
        g.output = quote! {
            u64
        };

        let body = g.body;
        g.body = quote! {
            let extra_filter = {
                #body
            };
            #db_fn(ctx, &tx, filter, extra_filter).await?
        };
    }

    g.no_tx = a.no_tx;
    gen_resolver(g)
}
