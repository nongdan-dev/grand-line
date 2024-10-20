use crate::prelude::*;
use proc_macro::TokenStream;
use quote::quote;

pub fn gen_count(_attr: TokenStream, _item: TokenStream) -> TokenStream {
    let a = parse_attr!(_attr);
    let g = parse_resolver!(q, _item, name_count(&a.model));
    let (a, mut g) = check_crud_io(a, g);

    let model = ty_model(&a.model);
    let model_filter = ty_filter(&a.model);
    let db_fn = db_count(&a.model);

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
            let f = filter_some_and!(#model, filter, extra_filter);
            #db_fn(&tx, f).await?
        };
    }

    g.no_tx = a.no_tx;
    gen_resolver(g)
}
