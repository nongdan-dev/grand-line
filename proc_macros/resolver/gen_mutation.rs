use crate::prelude::*;
use proc_macro::TokenStream;

pub fn gen_mutation(attr: TokenStream, item: TokenStream) -> TokenStream {
    let a = parse_attr!(attr);
    let mut g = parse_resolver!(ty_mutation, item);

    g.no_tx = a.no_tx;
    gen_resolver(g)
}
