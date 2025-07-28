mod utils;

#[allow(unused_imports)]
mod prelude {
    pub use crate::utils::*;
    pub use grand_line_macros::*;

    pub use heck::*;
    pub use maplit::*;
    pub use proc_macro::TokenStream;
    pub use proc_macro2::TokenStream as TokenStream2;
    pub use quote::*;
    pub use std::{
        collections::{HashMap, HashSet},
        fmt::Display,
    };
}

use crate::prelude::*;

#[proc_macro_derive(PartialEqString)]
pub fn partial_eq_string(input: TokenStream) -> TokenStream {
    gen_partial_eq_string(input)
}

#[proc_macro]
pub fn attr_default_flag(input: TokenStream) -> TokenStream {
    gen_attr_default_flag(input)
}

#[proc_macro]
pub fn attr_unwrap(input: TokenStream) -> TokenStream {
    gen_attr_unwrap(input)
}
