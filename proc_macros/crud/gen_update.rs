use crate::prelude::*;
use syn::parse_macro_input;

pub fn gen_update(attr: TokenStream, item: TokenStream) -> TokenStream {
    let a = parse_macro_input!(attr as MacroAttr);
    let g = parse_resolver!(ty_mutation, item, camel_str!(a.model, "Update"));
    let (a, mut g) = check_crud_io(a, g);
    g.no_tx = a.no_tx;

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
    }

    gen_resolver(g)
}
