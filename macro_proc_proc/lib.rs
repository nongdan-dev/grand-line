mod mods;

#[allow(unused_imports)]
mod prelude {
    pub use crate::mods::*;
    pub use macro_utils::*;

    pub use heck::*;
    pub use maplit::*;
    pub use proc_macro::TokenStream;
    pub use proc_macro2::TokenStream as Ts2;
    pub use quote::*;

    // common std
    pub use std::collections::{HashMap, HashSet};
    pub use std::error::Error;
    pub use std::fmt::Display;
    pub use std::sync::{Arc, LazyLock};
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
