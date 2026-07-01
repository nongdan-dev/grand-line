use crate::prelude::*;

pub fn gen_attr_default_flag(input: TokenStream) -> TokenStream {
    let f_str = parse_macro_input!(input as Ident).to_string();
    try_gen_attr_default_flag(&f_str).unwrap_or_else(|e| e.to_compile_error().into())
}

fn try_gen_attr_default_flag(f_str: &str) -> SynRes<TokenStream> {
    let f = format!("default_{f_str}").ts2_or_err()?;
    let r = quote! {
        pub fn #f() -> bool {
            cfg!(feature = #f_str)
        }
    };
    Ok(r.into())
}
