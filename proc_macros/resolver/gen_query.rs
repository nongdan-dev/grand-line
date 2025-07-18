use crate::prelude::*;
use proc_macro::TokenStream;

pub fn gen_query(attr: TokenStream, item: TokenStream) -> TokenStream {
    let a = parse_attr!(attr);
    let mut g = parse_resolver!(ty_query, item);

    g.no_tx = a.no_tx;
    gen_resolver(g)
}
