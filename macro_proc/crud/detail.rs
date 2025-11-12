use crate::prelude::*;

pub fn gen_detail(attr: TokenStream, item: TokenStream) -> TokenStream {
    let a = parse_macro_input!(attr as AttrParse);
    let r = parse_macro_input!(item as ResolverTyItem);
    let a = a.into_inner::<CrudAttr>("detail");
    let (mut r, ty, name) = r.init("query", "detail", &a.model);
    a.validate(&r);

    if !a.resolver_inputs {
        r.inputs = quote! {
            id: String,
        };
        r.inputs = push_include_deleted(r.inputs, !a.ra.no_include_deleted);
    }

    if !a.resolver_output {
        let output = ty_gql(&a.model);
        r.output = quote!(Option<#output>);

        let include_deleted = if !a.resolver_inputs && !a.ra.no_include_deleted {
            quote!(include_deleted)
        } else {
            quote!(None)
        };

        let body = r.body;
        let model = ts2!(a.model);
        r.body = quote! {
            #body
            #model::gql_detail(ctx, tx, &id, #include_deleted).await?
        }
    }

    ResolverTy::g(ty, name, a.ra, r)
}
