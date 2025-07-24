use crate::prelude::*;
use syn::parse_macro_input;

pub fn gen_create(attr: TokenStream, item: TokenStream) -> TokenStream {
    let a = parse_macro_input!(attr as MacroAttr);
    let mut g = parse_macro_input!(item as GenResolverTy);
    g.init(&a, "Mutation", "Create");
    check_crud_io(&a, &g);

    if !a.resolver_inputs {
        let data = pascal!(g.name);
        g.inputs = quote!(data: #data);
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
            am.insert(tx).await?.into()
        }
    }

    gen_resolver_ty(g)
}
