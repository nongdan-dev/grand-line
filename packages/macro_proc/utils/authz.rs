use crate::prelude::*;

pub fn gen_authz_row_filter(filter: &Ts2, enable: bool) -> Ts2 {
    if cfg!(feature = "authz") && enable {
        quote!(ctx.authz_row_graceful::<#filter>().await?)
    } else {
        quote!(None)
    }
}

pub fn gen_authz_err(enable: bool) -> Ts2 {
    if cfg!(feature = "authz") && enable {
        quote!(ctx.authz_err())
    } else {
        quote!(CoreDbErr::Db404)
    }
}
