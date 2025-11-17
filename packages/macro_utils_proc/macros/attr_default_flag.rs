use crate::prelude::*;

pub fn gen_attr_default_flag(input: TokenStream) -> TokenStream {
    let f_str = parse_macro_input!(input as Ident).to_string();
    let f = format!("default_{f_str}").ts2_or_panic();

    quote! {
        pub fn #f() -> bool {
            cfg!(feature = #f_str)
        }
    }
    .into()
}
