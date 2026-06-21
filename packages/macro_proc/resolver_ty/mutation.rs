use crate::prelude::*;

pub fn gen_mutation(attr: TokenStream, item: TokenStream) -> TokenStream {
    let a = parse_macro_input!(attr as AttrParse);
    let r = parse_macro_input!(item as ResolverTyItem);
    try_gen_mutation(a, r).unwrap_or_else(|e| e.to_compile_error().into())
}

fn try_gen_mutation(a: AttrParse, r: ResolverTyItem) -> SynRes<TokenStream> {
    let a = a.into_inner::<ResolverTyAttr>("mutation")?;
    let (r, ty, name) = r.init("mutation", "", "")?;
    ResolverTy::g(ty, name, a, r)
}
