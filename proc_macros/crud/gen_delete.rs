use crate::prelude::*;
use syn::parse_macro_input;

pub fn gen_delete(attr: TokenStream, item: TokenStream) -> TokenStream {
    let a = parse_macro_input!(attr as AttrParseX<CrudAttr>);
    let r = parse_macro_input!(item as ResolverTyItem);
    let a = a.attr(&r.gql_name, "delete");
    let (mut r, ty, name) = r.init("mutation", "delete", &a.model);
    check_crud_io(&a, &r);

    if !a.resolver_inputs {
        r.inputs = quote!(id: String);
    }

    if !a.resolver_output {
        let output = ty_gql(&a.model);
        r.output = quote!(#output);

        let body = r.body;
        let model = ts2!(a.model);
        r.body = quote! {
            #body
            #model::gql_delete(tx, &id).await?
        };
    }

    ResolverTy::g(ty, name, a.resolver_attr, r)
}
