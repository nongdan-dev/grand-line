use crate::prelude::*;
use syn::parse_macro_input;

pub fn gen_mutation(attr: TokenStream, item: TokenStream) -> TokenStream {
    let a = parse_macro_input!(attr as MacroAttr);
    let mut g = parse_resolver!(ty_mutation, item);

    g.no_tx = a.no_tx;
    gen_resolver(g)
}
