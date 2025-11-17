use crate::prelude::*;

pub fn expr_struct(item: TokenStream, suf: &str, wrap: &str, method: &str) -> TokenStream {
    let item2 = Into::<Ts2>::into(item.clone());
    let item = if !item2.to_string().trim().ends_with('}') {
        Into::<TokenStream>::into(quote!(#item2{}))
    } else {
        item
    };
    let item = parse_macro_input!(item as ExprStruct);

    let model = item.path.get_ident().to_token_stream();
    let name = format!("{model}{suf}").ts2_or_panic();

    let rest = item.rest.to_token_stream();
    let rest = if rest.to_string().trim().is_empty() {
        quote!(..Default::default())
    } else {
        quote!(..#rest)
    };

    let mut fields = vec![];
    for f in item.fields.iter() {
        let v = if let Expr::Lit(l) = &f.expr {
            if let Lit::Str(s) = &l.lit {
                let v = s.value();
                quote!(#v.to_owned())
            } else {
                l.to_token_stream()
            }
        } else {
            f.expr.to_token_stream()
        };
        let member = f.member.to_token_stream();
        let wrap = format!("{member}:{wrap}").ts2_or_panic();
        fields.push(quote!(#wrap(#v),));
    }

    let r = quote! {
        #name {
            #(#fields)*
            #rest
        }
    };

    if !method.is_empty() {
        let method = method.ts2_or_panic();
        quote!(#r.#method())
    } else {
        r
    }
    .into()
}
