mod gql;
mod sql;
mod utils;
pub use gql::*;
pub use sql::*;
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
    pub use macro_proc_proc::{PartialEqString, field_names};
    // common
    pub use async_graphql::MaybeUndefined as Undefined;
    pub use async_trait::async_trait;
    pub use serde::{Deserialize, Serialize};
    pub use thiserror::Error as ThisError;
    pub use tokio::sync::Mutex;
    // common std
    pub use std::collections::{HashMap, HashSet};
    pub use std::error::Error;
    pub use std::fmt::Display;
    pub use std::sync::{Arc, LazyLock};
}

#[cfg(not(feature = "no_re_exports"))]
pub use re_exports::*;

#[allow(unused_imports)]
pub(crate) mod prelude {
    pub use crate::*;
    pub use async_graphql::{extensions::*, *};
    pub use sea_orm::{entity::prelude::*, prelude::*, *};

    #[cfg(feature = "no_re_exports")]
    pub use re_exports::*;

    pub use async_graphql::Error as GraphQLError;
    pub use std::error::Error;
}
