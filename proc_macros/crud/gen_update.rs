use crate::prelude::*;
use syn::parse_macro_input;

pub fn gen_update(attr: TokenStream, item: TokenStream) -> TokenStream {
    let a = parse_macro_input!(attr as AttrParse);
    let r = parse_macro_input!(item as ResolverTyItem);
    let a = a.into_inner::<CrudAttr>("update");
    let (mut r, ty, name) = r.init("mutation", "update", &a.model);
    check_crud_io(&a, &r);

    if !a.resolver_inputs {
        let data = pascal!(name);
        r.inputs = quote! {
            id: String,
            data: #data,
        };
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
            am.update(tx).await?.into()
        }
    }

    ResolverTy::g(ty, name, a.resolver_attr, r)
}
