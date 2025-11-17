use crate::prelude::*;

pub fn gen_search(attr: TokenStream, item: TokenStream) -> TokenStream {
    let a = parse_macro_input!(attr as AttrParse);
    let r = parse_macro_input!(item as ResolverTyItem);
    let a = a.into_inner::<CrudAttr>("search");
    let (mut r, ty, name) = r.init("query", "search", &a.model);
    a.validate(&r);

    let filter = ty_filter(&a.model);
    let order_by = ty_order_by(&a.model);

    if !a.resolver_inputs {
        r.inputs = quote! {
            filter: Option<#filter>,
            order_by: Option<Vec<#order_by>>,
            page: Option<Pagination>,
        };
        r.inputs = push_include_deleted(r.inputs, !a.ra.no_include_deleted);
    }

    if !a.resolver_output {
        let output = ty_gql(&a.model);
        r.output = quote!(Vec<#output>);

        let include_deleted = if !a.resolver_inputs && !a.ra.no_include_deleted {
            quote!(include_deleted)
        } else {
            quote!(None)
        };

        let body = r.body;
        let model = a.model.ts2_or_panic();
        r.body = quote! {
            let (filter_extra, order_by_default): (Option<#filter>, Option<Vec<#order_by>>) = {
                #body
            };
            #model::gql_search(ctx, tx, None, filter, filter_extra, order_by, order_by_default, page, #include_deleted).await?
        };
    }

    ResolverTy::g(ty, name, a.ra, r)
}
