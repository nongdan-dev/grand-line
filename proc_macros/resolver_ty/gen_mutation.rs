use crate::prelude::*;
use syn::parse_macro_input;

pub fn gen_mutation(attr: TokenStream, item: TokenStream) -> TokenStream {
    let a = parse_macro_input!(attr as AttrParse);
    let r = parse_macro_input!(item as ResolverTyItem);
    let a = a.into_with_validate::<ResolverTyAttr>(&r.gql_name, "mutation");
    let (r, ty, name) = r.init("mutation", "", "");

    ResolverTy::g(ty, name, a, r)
}
