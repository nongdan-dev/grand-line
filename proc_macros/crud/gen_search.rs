use crate::prelude::*;
use syn::parse_macro_input;

pub fn gen_search(attr: TokenStream, item: TokenStream) -> TokenStream {
    let a = parse_macro_input!(attr as MacroAttr);
    let mut g = parse_macro_input!(item as GenResolverTy);
    g.init(&a, "Query", "Search");
    check_crud_io(&a, &g);

    let filter = ty_filter(&a.model);
    let order_by = ty_order_by(&a.model);

    if !a.resolver_inputs {
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
        let model = ts2!(a.model);
        g.body = quote! {
            let (filter_extra, order_by_default): (Option<#filter>, Option<Vec<#order_by>>) = {
                #body
            };
            #model::gql_search(ctx, tx, None, filter, filter_extra, order_by, order_by_default, page).await?
        };
    }

    gen_resolver_ty(g)
}
