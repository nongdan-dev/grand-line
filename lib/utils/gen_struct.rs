use crate::prelude::*;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote, ToTokens};
use syn::{parse_macro_input, Expr, ExprStruct, Lit};

pub fn gen_struct(
    item: TokenStream,
    suffix: &str,
    field_wrap: &str,
    return_wrap: &str,
    update: bool,
) -> TokenStream {
    let field_wrap: TokenStream2 = field_wrap.parse().unwrap();
    let item = parse_macro_input!(item as ExprStruct);

    let name = format_ident!(
        "{}{}",
        item.path.get_ident().to_token_stream().to_string(),
        suffix,
    );
    let rest = parse_rest(&item, !update);

    let mut fields = vec![];
    for f in item.fields.into_iter() {
        let m = f.member;
        if let Expr::Lit(l) = f.expr {
            if let Lit::Str(s) = l.lit {
                let v = s.value();
                fields.push(quote!(#m:#field_wrap(#v.to_string()),));
            } else {
                fields.push(quote!(#m:#field_wrap(#l),));
            }
        } else {
            let e = f.expr;
            fields.push(quote!(#m:#field_wrap(#e),));
        }
    }
    let mut r = quote! {
        #name {
            #(#fields)*
            #rest
        }
    };
    if return_wrap != "" {
        let rw: TokenStream2 = return_wrap.parse().unwrap();
        r = quote!(#rw(#r))
    }
    r.into()
}
