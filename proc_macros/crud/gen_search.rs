use crate::prelude::*;
use syn::parse_macro_input;

pub fn gen_search(attr: TokenStream, item: TokenStream) -> TokenStream {
    let a = parse_macro_input!(attr as AttrParse);
    let r = parse_macro_input!(item as ResolverTyItem);
    let a = a.into_with_validate::<CrudAttr>(&r.gql_name, "search");
    let (mut r, ty, name) = r.init("query", "search", &a.model);
    check_crud_io(&a, &r);

    let filter = ty_filter(&a.model);
    let order_by = ty_order_by(&a.model);

    if !a.resolver_inputs {
        r.inputs = quote! {
            filter: Option<#filter>,
            order_by: Option<Vec<#order_by>>,
            page: Option<Pagination>,
        };
    }

    if !a.resolver_output {
        let output = ty_gql(&a.model);
        r.output = quote!(Vec<#output>);

        let body = r.body;
        let model = ts2!(a.model);
        r.body = quote! {
            let (filter_extra, order_by_default): (Option<#filter>, Option<Vec<#order_by>>) = {
                #body
            };
            #model::gql_search(ctx, tx, None, filter, filter_extra, order_by, order_by_default, page).await?
        };
    }

    ResolverTy::g(ty, name, a.resolver_attr, r)
}
