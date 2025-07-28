mod context;
mod utils;
pub use context::*;
pub use utils::*;

pub use grand_line_macros::active_value;
pub use grand_line_proc_macros::{
    GrandLineModel, active_create, active_model, active_update, count, create, delete, detail,
    enunn, filter, filter_some, input, model, mutation, order_by, order_by_some, query, search,
    update,
};

mod re_exports {
    pub use async_graphql;
    pub use async_trait;
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
    pub use async_graphql_axum;
    #[cfg(feature = "axum")]
    pub use axum;
    #[cfg(feature = "axum")]
    pub use tower;
    #[cfg(feature = "axum")]
    pub use tower_http;

    #[cfg(feature = "tracing")]
    pub use tracing;
    #[cfg(feature = "tracing")]
    pub use tracing_subscriber;

    pub use grand_line_proc_proc_macros::PartialEqString;

    pub use async_graphql::MaybeUndefined as Undefined;
    pub use std::{
        collections::{HashMap, HashSet},
        error::Error,
        sync::{Arc, LazyLock},
    };
    pub use tokio::sync::Mutex;
}

#[cfg(not(feature = "no_re_exports"))]
pub use re_exports::*;

pub(crate) mod prelude {
    pub use crate::*;
    pub use async_trait::async_trait;
    pub use sea_orm::prelude::*;
    pub use sea_orm::*;

    #[cfg(feature = "no_re_exports")]
    pub use re_exports::*;
}
