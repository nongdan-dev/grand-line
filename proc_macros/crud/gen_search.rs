use crate::prelude::*;
use syn::parse_macro_input;

pub fn gen_search(attr: TokenStream, item: TokenStream) -> TokenStream {
    let a = parse_macro_input!(attr as MacroAttr);
    let g = parse_resolver!(ty_query, item, camel_str!(a.model, "Search"));
    let (a, mut g) = check_crud_io(a, g);
    g.no_tx = a.no_tx;

    if !a.resolver_inputs {
        let filter = ty_filter(&a.model);
        let order_by = ty_order_by(&a.model);
        g.inputs = quote! {
            filter: Option<#filter>,
            order_by: Option<Vec<#order_by>>,
            page: Option<Pagination>,
        };
    }

    if !a.resolver_output {
        let output = ty_gql(&a.model);
        g.output = quote!(Vec<#output>);

        let body = g.body;
        let db_fn = ts2!(a.model, "::gql_search");
        g.body = quote! {
            let (extra_filter, default_order_by) = {
                #body
            };
            #db_fn(ctx, tx, filter, extra_filter, order_by, default_order_by, page).await?
        };
    }

    gen_resolver(g)
}
