use crate::prelude::*;
use syn::{Ident, parse_macro_input};

pub fn gen_attr_default_flag(input: TokenStream) -> TokenStream {
    let f_str = parse_macro_input!(input as Ident).to_string();
    let f = ts2!("default_", f_str);

    quote! {
        pub fn #f() -> bool {
            let v = false;
            #[cfg(feature = #f_str)]
            let v = true;
            v
        }
    }
    .into()
}
