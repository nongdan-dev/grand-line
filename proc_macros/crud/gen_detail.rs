use crate::prelude::*;
use syn::parse_macro_input;

pub fn gen_detail(attr: TokenStream, item: TokenStream) -> TokenStream {
    let a = parse_macro_input!(attr as MacroAttr);
    let mut g = parse_macro_input!(item as GenResolver);
    g.init(&a, "Query", "Detail");
    check_crud_io(&a, &g);

    if !a.resolver_inputs {
        g.inputs = quote!(id: String);
    }

    if !a.resolver_output {
        let output = ty_gql(&a.model);
        g.output = quote!(Option<#output>);

        let body = g.body;
        let model = ts2!(a.model);
        g.body = quote! {
            #body
            #model::gql_detail(ctx, tx, &id).await?
        }
    }

    gen_resolver(g)
}
