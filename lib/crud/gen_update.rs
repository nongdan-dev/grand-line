use crate::prelude::*;
use proc_macro::TokenStream;
use quote::{format_ident, quote};

pub fn gen_update(_attr: TokenStream, _item: TokenStream) -> TokenStream {
    let a = parse_attr!(_attr);
    let g = parse_resolver!(m, _item, name_update(&a.model));
    let (a, mut g) = check_crud_io(a, g);

    let output = ty_output(&a.model);
    let ty = ty_input(&g.name);

    if !a.resolver_inputs {
        g.inputs = quote! {
            id: String,
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
