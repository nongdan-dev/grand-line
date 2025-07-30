use crate::prelude::*;
use syn::{Expr, ExprStruct, Lit, parse_macro_input};

pub fn gen_struct(item: TokenStream, suf: &str, fw: &str, rw: &str) -> TokenStream {
    let mut item = Into::<TokenStream2>::into(item)
        .to_string()
        .trim()
        .to_owned();
    if !item.ends_with("}") {
        item = item + "{}";
    }
    let item = Into::<TokenStream>::into(ts2!(item));
    let item = parse_macro_input!(item as ExprStruct);

    let model = item.path.get_ident().to_token_stream();
    let name = ts2!(model, suf);

    let rest = item.rest.to_token_stream();
    let rest = if str!(rest).trim() == "" {
        ts2!("..Default::default()")
    } else {
        rest
    };

    let mut fields = vec![];
    for f in item.fields.into_iter() {
        let v = if let Expr::Lit(l) = f.expr {
            if let Lit::Str(s) = l.lit {
                let v = s.value();
                quote!(#v.to_string())
            } else {
                l.to_token_stream()
            }
        } else {
            f.expr.to_token_stream()
        };
        let fw = ts2!(f.member.to_token_stream(), ":", fw);
        fields.push(quote!(#fw(#v)));
    }

    let r = quote! {
        #name {
            #(#fields,)*
            #rest
        }
    };

    if rw != "" {
        let rwrap = ts2!(model, "::", rw);
        quote!(#rwrap(#r))
    } else {
        r
    }
    .into()
}
