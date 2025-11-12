use crate::prelude::*;

pub fn gen_delete(attr: TokenStream, item: TokenStream) -> TokenStream {
    let a = parse_macro_input!(attr as AttrParse);
    let r = parse_macro_input!(item as ResolverTyItem);
    let a = a.into_inner::<CrudAttr>("delete");
    let (mut r, ty, name) = r.init("mutation", "delete", &a.model);
    a.validate(&r);

    if !a.resolver_inputs {
        r.inputs = quote! {
            id: String,
        };
        if !a.no_permanent_delete {
            let inputs = r.inputs;
            r.inputs = quote! {
                #inputs
                permanent: Option<bool>,
            }
        }
    }

    if !a.resolver_output {
        let output = ty_gql(&a.model);
        r.output = quote!(#output);

        let permanent = if !a.resolver_inputs && !a.no_permanent_delete {
            quote!(permanent)
        } else {
            quote!(None)
        };

        let body = r.body;
        let model = ts2!(a.model);
        r.body = quote! {
            #body
            #model::gql_delete(tx, &id, #permanent).await?
        };
    }

    ResolverTy::g(ty, name, a.ra, r)
}
