use crate::prelude::*;

pub fn expr_struct(item: TokenStream, suf: &str, wrap: &str, method: &str) -> TokenStream {
    let item2 = Into::<Ts2>::into(item.clone());
    let item = if !item2.to_string().trim().ends_with('}') {
        Into::<TokenStream>::into(quote!(#item2{}))
    } else {
        item
    };
    let item = parse_macro_input!(item as ExprStruct);
    try_expr_struct(item, suf, wrap, method).unwrap_or_else(|e| e.to_compile_error().into())
}

fn try_expr_struct(item: ExprStruct, suf: &str, wrap: &str, method: &str) -> SynRes<TokenStream> {
    let r = build_expr_struct(&item, suf, wrap)?;

    Ok(if !method.is_empty() {
        let method = method.ts2_or_err()?;
        quote!(#r.#method())
    } else {
        r
    }
    .into())
}

pub fn expr_struct_am_wrapper(item: TokenStream, suf: &str, op_ty: &str) -> TokenStream {
    let item2 = Into::<Ts2>::into(item.clone());
    let item = if !item2.to_string().trim().ends_with('}') {
        Into::<TokenStream>::into(quote!(#item2{}))
    } else {
        item
    };
    let item = parse_macro_input!(item as ExprStruct);
    try_expr_struct_am_wrapper(item, suf, op_ty).unwrap_or_else(|e| e.to_compile_error().into())
}

fn try_expr_struct_am_wrapper(item: ExprStruct, suf: &str, op_ty: &str) -> SynRes<TokenStream> {
    let entity = item.path.get_ident().to_token_stream();
    let r = build_expr_struct(&item, suf, "Set")?;
    let op_ty = op_ty.ts2_or_err()?;
    Ok(quote!(ActiveModelWrapper::<#op_ty, #entity, _>::new(#r)).into())
}

fn build_expr_struct(item: &ExprStruct, suf: &str, wrap: &str) -> SynRes<Ts2> {
    let model = item.path.get_ident().to_token_stream();
    let name = format!("{model}{suf}").ts2_or_err()?;

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
        let wrap = format!("{member}:{wrap}").ts2_or_err()?;
        fields.push(quote!(#wrap(#v),));
    }

    Ok(quote! {
        #name {
            #(#fields)*
            #rest
        }
    })
}
