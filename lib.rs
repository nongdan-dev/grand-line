#![allow(unused_imports, ambiguous_glob_reexports)]

mod core;

#[cfg(feature = "test_utils")]
mod test_utils;

#[cfg(feature = "axum")]
mod http;

#[cfg(feature = "authenticate")]
mod authenticate;

pub mod prelude {
    pub use {
        macro_proc::*,
        macro_utils::{am_value, err},
        macro_utils_proc::{PartialEqString, field_names},
    };

    pub use {
        crate::core::*, async_graphql, chrono, sea_orm, serde, serde_json, serde_with, sqlx,
        thiserror, tokio, ulid,
    };
    #[cfg(feature = "tracing")]
    pub use {tracing, tracing_subscriber};

    #[cfg(feature = "test_utils")]
    pub use {crate::test_utils::*, macro_utils::*, pretty_assertions::assert_eq as pretty_eq};

    #[cfg(feature = "axum")]
    pub use {crate::http::*, async_graphql_axum, axum, cookie, tower, tower_http};

    #[cfg(feature = "authenticate")]
    pub use {crate::authenticate::*, argon2, base64, rand, rand_core, serde_qs, validator};

    pub use {
        async_graphql::{extensions::*, *},
        sea_orm::{entity::prelude::*, prelude::*, *},
    };

    // alias explicit ambiguous
    pub use async_graphql::{Error as GraphQLErr, MaybeUndefined as Undefined, Schema, Value};
    pub use async_trait::async_trait;
    pub use sea_orm::{
        Schema as DbSchema,
        sea_query::{IntoCondition, SimpleExpr},
    };
    pub use serde::{Deserialize, Serialize};
    pub use serde_json::Error as JsonErr;
    pub use thiserror::Error as ThisErr;
    pub use tokio::sync::Mutex;
    macro_utils::use_common_std!();
}
