use crate::prelude::*;
use syn::{ExprStruct, parse_macro_input};

pub fn gen_attr_unwrap(item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as ExprStruct);
    let struk = item.path.get_ident().to_token_stream();

    let mut fields = vec![];
    for f in item.fields.into_iter() {
        let k = f.member.to_token_stream();
        let k_str = str!(k);
        let k_default = snake!("default", k);
        let e = f.expr.to_token_stream();
        let e_str = str!(e);
        let v = if e_str == "bool" || e_str == "parse" {
            quote!(a.#e(#k_str).unwrap_or_else(#k_default))
        } else {
            f.expr.to_token_stream()
        };
        fields.push(quote!(#k: #v));
    }

    quote! {
        #struk {
            #(#fields),*
        }
    }
    .into()
}
