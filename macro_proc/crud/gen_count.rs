use crate::prelude::*;
use syn::parse_macro_input;

pub fn gen_count(attr: TokenStream, item: TokenStream) -> TokenStream {
    let a = parse_macro_input!(attr as AttrParse);
    let r = parse_macro_input!(item as ResolverTyItem);
    let a = a.into_inner::<CrudAttr>("count");
    let (mut r, ty, name) = r.init("query", "count", &a.model);
    check_crud_io(&a, &r);

    let filter = ty_filter(&a.model);

    if !a.resolver_inputs {
        r.inputs = quote!(filter: Option<#filter>);
    }

    if !a.resolver_output {
        r.output = quote!(u64);

        let body = r.body;
        let model = ts2!(a.model);
        r.body = quote! {
            let filter_extra: Option<#filter> = {
                #body
            };
            #model::gql_count(tx, filter, filter_extra).await?
        };
    }

    ResolverTy::g(ty, name, a.resolver_attr, r)
}
