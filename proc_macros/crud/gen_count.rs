use crate::prelude::*;
use syn::parse_macro_input;

pub fn gen_count(attr: TokenStream, item: TokenStream) -> TokenStream {
    let a = parse_macro_input!(attr as MacroAttr);
    let g = parse_resolver!(ty_query, item, camel_str!(a.model, "Count"));
    let (a, mut g) = check_crud_io(a, g);
    g.no_tx = a.no_tx;

    if !a.resolver_inputs {
        let filter = ty_filter(&a.model);
        g.inputs = quote!(filter: Option<#filter>);
    }

    if !a.resolver_output {
        g.output = quote!(u64);

        let body = g.body;
        let db_fn = ts2!(a.model, "::gql_count");
        g.body = quote! {
            let extra_filter = {
                #body
            };
            #db_fn(ctx, tx, filter, extra_filter).await?
        };
    }

    gen_resolver(g)
}
