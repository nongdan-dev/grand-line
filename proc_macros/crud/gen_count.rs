use crate::prelude::*;
use syn::parse_macro_input;

pub fn gen_count(attr: TokenStream, item: TokenStream) -> TokenStream {
    let a = parse_macro_input!(attr as MacroAttr);
    let mut g = parse_macro_input!(item as GenResolverTy);
    g.init(&a, "Query", "Count");
    check_crud_io(&a, &g);

    let filter = ty_filter(&a.model);

    if !a.resolver_inputs {
        g.inputs = quote!(filter: Option<#filter>);
    }

    if !a.resolver_output {
        g.output = quote!(u64);

        let body = g.body;
        let model = ts2!(a.model);
        g.body = quote! {
            let filter_extra: Option<#filter> = {
                #body
            };
            #model::gql_count(tx, filter, filter_extra).await?
        };
    }

    gen_resolver_ty(g)
}
