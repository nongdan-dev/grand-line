use crate::prelude::*;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

pub fn gen_delete(_attr: TokenStream, _item: TokenStream) -> TokenStream {
    let a = parse_attr!(_attr);
    let g = parse_resolver!(m, _item, name_delete(&a.model));
    let (a, mut g) = check_crud_io(a, g);

    let output = ty_output(&a.model);
    let db_fn: TokenStream2 = format!("{}::gql_delete", a.model).parse().unwrap();

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
