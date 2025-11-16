use crate::prelude::*;

pub fn gen_attr_unwrap(item: TokenStream, default: bool) -> TokenStream {
    let item = parse_macro_input!(item as ExprStruct);
    let name = item.path.get_ident().to_token_stream();

    let fields = item
        .fields
        .iter()
        .map(|f| {
            let k = f.member.to_token_stream();
            let k_str = s!(k);
            let e = f.expr.to_token_stream();
            let e_str = s!(e);
            let v = if e_str == "bool" || e_str == "parse" {
                if default {
                    quote!(a.#e(#k_str).unwrap_or_default())
                } else {
                    let k_default = snake!("default", k);
                    quote!(a.#e(#k_str).unwrap_or_else(#k_default))
                }
            } else {
                f.expr.to_token_stream()
            };
            let attrs = &f.attrs;
            quote! {
                #(#attrs)*
                #k: #v,
            }
        })
        .collect::<Vec<_>>();

    quote! {
        #name {
            #(#fields)*
        }
    }
    .into()
}
