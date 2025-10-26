/// Internal macro to reuse common imports.
#[macro_export]
macro_rules! use_common_std {
    () => {
        pub use std::any::{Any, TypeId};
        pub use std::collections::{HashMap, HashSet};
        pub use std::error::Error;
        pub use std::fmt::Display;
        pub use std::hash::{Hash, Hasher};
        pub use std::sync::{Arc, LazyLock};
    };
}

#[macro_export]
macro_rules! use_common_macro_utils {
    () => {
        pub use heck::*;
        pub use maplit::*;
        pub use proc_macro2::TokenStream as Ts2;
        pub use quote::*;
    };
}
