use crate::prelude::*;
use syn::parse_macro_input;

pub fn gen_update(attr: TokenStream, item: TokenStream) -> TokenStream {
    let a = parse_macro_input!(attr as MacroAttr);
    let mut g = parse_macro_input!(item as GenResolverTy);
    g.init(&a, "Mutation", "Update");
    check_crud_io(&a, &g);

    if !a.resolver_inputs {
        let data = pascal!(g.name);
        g.inputs = quote! {
            id: String,
            data: #data,
        };
    }

    if !a.resolver_output {
        let output = ty_gql(&a.model);
        g.output = quote!(#output);

        let body = g.body;
        let am = ty_active_model(&a.model);
        g.body = quote! {
            let am: #am = {
                #body
            };
            am.update(tx).await?.into()
        }
    }

    gen_resolver_ty(g)
}
