use crate::prelude::*;
use proc_macro::TokenStream;

pub fn gen_query(_attr: TokenStream, _item: TokenStream) -> TokenStream {
    let a = parse_attr!(_attr);
    let mut g = parse_resolver!(q, _item);

    g.no_tx = a.no_tx;
    gen_resolver(g)
}
