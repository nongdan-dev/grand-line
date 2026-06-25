use crate::prelude::*;

pub fn gen_authz_row_filter(filter: &Ts2, enable: bool) -> Ts2 {
    if !cfg!(feature = "authz") || !enable {
        return quote!(None);
    }
    quote!(ctx.authz_row_graceful::<#filter>().await?)
}

pub fn gen_authz_row_filter_var(filter: &Ts2, enable: bool) -> (Ts2, Ts2) {
    if !cfg!(feature = "authz") || !enable {
        return (quote!(None), "".into_token_stream());
    }
    let var = unique_ident();
    let authz_row_filter = gen_authz_row_filter(filter, enable);
    (var.clone(), quote!(let #var = #authz_row_filter;))
}

pub fn gen_authz_err(enable: bool) -> Ts2 {
    if cfg!(feature = "authz") && enable {
        quote!(ctx.authz_err())
    } else {
        quote!(CoreDbErr::Db404)
    }
}
