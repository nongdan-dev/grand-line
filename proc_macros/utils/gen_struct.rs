use crate::prelude::*;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{ToTokens, quote};
use syn::{Expr, ExprStruct, Lit, parse_macro_input};

pub fn gen_struct(
    item: TokenStream,
    suffix: &str,
    field_wrap: &str,
    return_wrap: &str,
) -> TokenStream {
    let field_wrap = field_wrap.parse::<TokenStream2>().unwrap();
    let item = parse_macro_input!(item as ExprStruct);

    let name = format!("{}{}", item.path.get_ident().to_token_stream(), suffix,)
        .parse::<TokenStream2>()
        .unwrap();
    let rest = parse_rest(&item, true);

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
        let rw = return_wrap.parse::<TokenStream2>().unwrap();
        r = quote!(#rw(#r))
    }
    r.into()
}
