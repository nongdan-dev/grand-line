mod macros;

#[allow(unused_imports)]
mod prelude {
    pub use crate::macros::*;
    pub use _macro_utils::*;
    pub use proc_macro::TokenStream;
    use_common_macro_utils!();
    use_common_std!();
}
use crate::prelude::*;

#[proc_macro_derive(PartialEqString)]
pub fn partial_eq_string(input: TokenStream) -> TokenStream {
    gen_partial_eq_string(input)
}

#[proc_macro_attribute]
pub fn field_names(attr: TokenStream, input: TokenStream) -> TokenStream {
    gen_field_names(attr, input)
}

#[proc_macro]
pub fn attr_default_flag(input: TokenStream) -> TokenStream {
    gen_attr_default_flag(input)
}

#[proc_macro]
pub fn attr_unwrap_default(input: TokenStream) -> TokenStream {
    gen_attr_unwrap(input, true)
}

#[proc_macro]
pub fn attr_unwrap_or_else(input: TokenStream) -> TokenStream {
    gen_attr_unwrap(input, false)
}
