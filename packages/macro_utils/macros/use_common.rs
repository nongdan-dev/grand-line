/// Internal macro to reuse common imports.
#[macro_export]
macro_rules! use_common_std {
    () => {
        pub use std::any::{Any, TypeId};
        pub use std::collections::{HashMap, HashSet};
        pub use std::error::Error;
        pub use std::fmt::{Debug, Display, Error as FmtErr, Formatter, Result as FmtRes};
        pub use std::future::Future;
        pub use std::hash::{Hash, Hasher};
        pub use std::marker::PhantomData;
        pub use std::str::FromStr;
        pub use std::sync::{Arc, LazyLock};
        pub type ArcAny = Arc<dyn Any + Send + Sync>;
    };
}

#[macro_export]
macro_rules! use_common_macro_utils {
    () => {
        pub use heck::*;
        pub use maplit::*;
        pub use proc_macro2::{Span, TokenStream as Ts2};
        pub use quote::*;
        pub use syn::{
            Error as SynErr, Result as SynRes, parse::*, punctuated::*, spanned::Spanned,
            token::Paren, *,
        };
    };
}
