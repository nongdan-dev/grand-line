use crate::prelude::*;

pub fn gen_mutation(attr: TokenStream, item: TokenStream) -> TokenStream {
    let a = parse_macro_input!(attr as AttrParse);
    let r = parse_macro_input!(item as ResolverTyItem);
    let a = a.into_inner::<ResolverTyAttr>("mutation");
    let (r, ty, name) = r.init("mutation", "", "");

    ResolverTy::g(ty, name, a, r)
}
