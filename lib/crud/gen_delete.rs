use crate::prelude::*;
use proc_macro::TokenStream;
use quote::quote;

pub fn gen_delete(_attr: TokenStream, _item: TokenStream) -> TokenStream {
    let a = parse_attr!(_attr);
    let g = parse_resolver!(m, _item, name_delete(&a.model));
    let (a, mut g) = check_crud_io(a, g);

    let model = ty_model(&a.model);
    let db_fn = db_delete(&a.model);

    if !a.resolver_inputs {
        g.inputs = quote! {
            id: String,
        };
    }

    if !a.resolver_output {
        g.output = quote! {
            Option<#model>
        };

        let body = g.body;
        g.body = quote! {
            #body
            #db_fn(&tx, id).await?
        };
    }

    g.no_tx = a.no_tx;
    gen_resolver(g)
}
