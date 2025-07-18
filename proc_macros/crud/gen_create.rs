use crate::prelude::*;
use proc_macro::TokenStream;
use quote::quote;

pub fn gen_create(attr: TokenStream, item: TokenStream) -> TokenStream {
    let a = parse_attr!(attr);
    let g = parse_resolver!(ty_mutation, item, gql_create(&a.model));
    let (a, mut g) = check_crud_io(a, g);

    let output = ty_gql(&a.model);
    let ty = ty_input(&g.name);

    if !a.resolver_inputs {
        g.inputs = quote! {
            data: #ty,
        };
    }

    if !a.resolver_output {
        g.output = quote! {
            #output
        };
    }

    g.no_tx = a.no_tx;
    gen_resolver(g)
}
