mod db;
mod graphql;
mod utils;
pub use db::*;
pub use graphql::*;
pub use utils::*;

pub use macro_proc::*;
pub use macro_utils::{am_value, err};

mod re_exports {
    pub use async_graphql;
    pub use async_graphql_axum;
    pub use axum;
    pub use chrono;
    pub use sea_orm;
    pub use serde;
    pub use serde_json;
    pub use serde_with;
    pub use sqlx;
    pub use thiserror;
    pub use tokio;
    pub use tower;
    pub use tower_http;
    pub use ulid;

    #[cfg(feature = "tracing")]
    pub use {tracing, tracing_subscriber};

    // utils
    pub use macro_utils_proc::{PartialEqString, field_names};
    // common
    pub use async_graphql::MaybeUndefined as Undefined;
    pub use async_trait::async_trait;
    pub use sea_orm::sea_query::{IntoCondition, SimpleExpr};
    pub use serde::{Deserialize, Serialize};
    pub use serde_json::Error as JsonErr;
    pub use thiserror::Error as ThisErr;
    pub use tokio::sync::Mutex;

    macro_utils::use_common_std!();
}

#[cfg(not(feature = "no_re_exports"))]
pub use re_exports::*;

#[allow(unused_imports)]
pub mod macro_prelude {
    #[cfg(feature = "no_re_exports")]
    pub use sea_orm::sea_query::{IntoCondition, SimpleExpr};
    pub use sea_orm::{entity::prelude::*, prelude::*, *};
}

#[allow(unused_imports, ambiguous_glob_reexports)]
pub mod prelude {
    pub use crate::*;
    pub use async_graphql::{extensions::*, *};
    pub use sea_orm::{entity::prelude::*, prelude::*, *};

    #[cfg(feature = "no_re_exports")]
    pub use re_exports::*;

    // alias and explicit use to avoid ambiguous
    pub use async_graphql::{Error as GraphQLErr, Schema, Value};
    pub use sea_orm::Schema as DbSchema;
    pub use std::any::Any;
    pub use std::error::Error;
}
