use crate::prelude::*;
use syn::parse_macro_input;

pub fn gen_delete(attr: TokenStream, item: TokenStream) -> TokenStream {
    let a = parse_macro_input!(attr as MacroAttr);
    let g = parse_resolver!(ty_mutation, item, camel_str!(a.model, "Delete"));
    let (a, mut g) = check_crud_io(a, g);
    g.no_tx = a.no_tx;

    if !a.resolver_inputs {
        g.inputs = quote!(id: String);
    }

    if !a.resolver_output {
        let output = ty_gql(&a.model);
        g.output = quote!(Option<#output>);

        let body = g.body;
        let db_fn = ts2!(a.model, "::gql_delete");
        g.body = quote! {
            #body
            #db_fn(ctx, tx, &id).await?
        };
    }

    gen_resolver(g)
}
