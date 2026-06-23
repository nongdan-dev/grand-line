use crate::prelude::*;

pub fn gen_count(attr: TokenStream, item: TokenStream) -> TokenStream {
    let a = parse_macro_input!(attr as AttrParse);
    let r = parse_macro_input!(item as ResolverTyItem);
    try_gen_count(a, r).unwrap_or_else(|e| e.to_compile_error().into())
}

fn try_gen_count(attr: AttrParse, r: ResolverTyItem) -> SynRes<TokenStream> {
    let a = attr.into_inner::<CrudAttr>("count")?;
    let (mut r, ty, name) = r.init("query", "count", &a.model)?;
    a.validate(&r)?;

    let filter = ty_filter(&a.model)?;

    if !a.resolver_inputs {
        r.inputs = quote! {
            filter: Option<#filter>,
        };
        r.inputs = push_include_deleted(r.inputs, a.ra.include_deleted);
    }

    if !a.resolver_output {
        r.output = quote!(u64);

        let body = r.body;
        let model = a.model.ts2_or_err()?;
        let include_deleted = get_include_deleted(!a.resolver_inputs && a.ra.include_deleted);
        r.body = quote! {
            let filter_extra: Option<#filter> = {
                #body
            };
            #model::gql_count(tx, filter, filter_extra, #include_deleted).await?
        };
    }

    ResolverTy::g(ty, name, a.ra, r)
}
