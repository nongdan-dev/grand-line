use crate::prelude::*;
use syn::parse_macro_input;

pub fn gen_query(attr: TokenStream, item: TokenStream) -> TokenStream {
    let a = parse_macro_input!(attr as MacroAttr);
    let mut g = parse_macro_input!(item as GenResolverTy);
    g.init(&a, "Query", "");

    gen_resolver_ty(g)
}
