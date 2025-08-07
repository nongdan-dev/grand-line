mod context;
mod utils;
pub use context::*;
pub use utils::*;

pub use macro_proc::{
    active_model, am_create, am_update, count, create, delete, detail, filter, filter_some,
    gql_enum, gql_input, model, mutation, order_by, order_by_some, query, search, update,
};
pub use macro_utils::am_value;

mod re_exports {
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

    #[cfg(feature = "axum")]
    pub use {async_graphql_axum, axum, tower, tower_http};

    #[cfg(feature = "tracing")]
    pub use {tracing, tracing_subscriber};

    // utils
    pub use macro_proc_proc::{PartialEqString, field_names};
    // common
    pub use async_graphql::MaybeUndefined as Undefined;
    pub use async_trait::async_trait;
    pub use tokio::sync::Mutex;
    // common std
    pub use std::collections::{HashMap, HashSet};
    pub use std::error::Error;
    pub use std::sync::{Arc, LazyLock};
}

#[cfg(not(feature = "no_re_exports"))]
pub use re_exports::*;

pub(crate) mod prelude {
    pub use crate::*;
    pub use sea_orm::prelude::*;
    pub use sea_orm::*;

    #[cfg(feature = "no_re_exports")]
    pub use re_exports::*;
}
