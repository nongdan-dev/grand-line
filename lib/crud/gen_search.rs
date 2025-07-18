use crate::prelude::*;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

pub fn gen_search(_attr: TokenStream, _item: TokenStream) -> TokenStream {
    let a = parse_attr!(_attr);
    let g = parse_resolver!(q, _item, name_search(&a.model));
    let (a, mut g) = check_crud_io(a, g);

    let output = ty_output(&a.model);
    let model_filter = ty_filter(&a.model);
    let model_order_by = ty_order_by(&a.model);
    let db_fn: TokenStream2 = format!("{}::gql_search", a.model).parse().unwrap();

    if !a.resolver_inputs {
        g.inputs = quote! {
            filter: Option<#model_filter>,
            order_by: Option<Vec<#model_order_by>>,
            page: Option<Pagination>,
        };
    }

    if !a.resolver_output {
        g.output = quote! {
            Vec<#output>
        };

        let body = g.body;
        g.body = quote! {
            let (extra_filter, default_order_by) = {
                #body
            };
            #db_fn(ctx, &tx, filter, extra_filter, order_by, default_order_by, page).await?
        };
    }

    g.no_tx = a.no_tx;
    gen_resolver(g)
}
