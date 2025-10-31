mod db;
mod graphql;
mod utils;

pub use crate::db::*;
pub use crate::graphql::*;
pub use crate::utils::*;
pub use _macro_proc::*;
pub use _macro_utils::{am_value, err};

#[cfg(feature = "authenticate")]
mod authenticate;
#[cfg(feature = "authenticate")]
pub use authenticate::*;

#[cfg(feature = "axum")]
mod http;
#[cfg(feature = "axum")]
pub use http::*;

mod reexport {
    pub use _macro_utils_proc::{PartialEqString, field_names};
    pub use async_graphql;
    pub use chrono;
    pub use sea_orm;
    pub use serde;
    pub use serde_json;
    pub use serde_with;
    pub use sqlx;
    pub use thiserror;
    pub use tokio;
    pub use ulid;
    #[cfg(feature = "authenticate")]
    pub use {argon2, base64, rand, rand_core, serde_qs, validator};
    #[cfg(feature = "axum")]
    pub use {async_graphql_axum, axum, cookie, tower, tower_http};
    #[cfg(feature = "tracing")]
    pub use {tracing, tracing_subscriber};
}
#[cfg(not(feature = "no_reexport_dep"))]
pub use reexport::*;

mod alias {
    pub use async_graphql::MaybeUndefined as Undefined;
    pub use async_trait::async_trait;
    pub use sea_orm::sea_query::{IntoCondition, SimpleExpr};
    pub use serde::{Deserialize, Serialize};
    pub use serde_json::Error as JsonErr;
    pub use thiserror::Error as ThisErr;
    pub use tokio::sync::Mutex;
    _macro_utils::use_common_std!();
}
#[cfg(not(feature = "no_reexport_alias"))]
pub use alias::*;

#[allow(unused_imports, ambiguous_glob_reexports)]
pub mod prelude {
    pub use async_graphql::{extensions::*, *};
    pub use sea_orm::{entity::prelude::*, prelude::*, *};

    pub use crate::alias::*;
    pub use crate::reexport::*;
    pub use crate::*;

    // explicit with alias to fix ambiguous
    pub use async_graphql::{Error as GraphQLErr, Schema, Value};
    pub use sea_orm::Schema as DbSchema;
    pub use std::any::Any;
    pub use std::error::Error;
}
