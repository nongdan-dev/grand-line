use crate::prelude::*;
use proc_macro::TokenStream;

pub fn gen_mutation(_attr: TokenStream, _item: TokenStream) -> TokenStream {
    let a = parse_attr!(_attr);
    let mut g = parse_resolver!(m, _item);

    g.no_tx = a.no_tx;
    gen_resolver(g)
}
