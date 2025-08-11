use crate::prelude::*;
use syn::parse_macro_input;

pub fn gen_create(attr: TokenStream, item: TokenStream) -> TokenStream {
    let a = parse_macro_input!(attr as AttrParse);
    let r = parse_macro_input!(item as ResolverTyItem);
    let a = a.into_inner::<CrudAttr>("create");
    let (mut r, ty, name) = r.init("mutation", "create", &a.model);
    a.validate(&r);

    if !a.resolver_inputs {
        let data = pascal!(name);
        r.inputs = quote!(data: #data);
    }

    if !a.resolver_output {
        let output = ty_gql(&a.model);
        r.output = quote!(#output);

        let body = r.body;
        let am = ty_active_model(&a.model);
        r.body = quote! {
            let am: #am = {
                #body
            };
            am.insert(tx).await?.into()
        }
    }

    ResolverTy::g(ty, name, a.ra, r)
}
