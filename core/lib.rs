mod db;
mod graphql;
mod utils;

pub mod export {
    pub use crate::{db::*, graphql::*, utils::*};
    pub use _proc::*;
}

pub mod reexport {
    pub use {
        _utils::{maplit, strum, strum_macros},
        _utils_proc::{PartialEqString, field_names},
        async_graphql, async_trait, chrono, sea_orm, serde, serde_json, serde_with, sqlx,
        thiserror, tokio, ulid,
    };
}

#[allow(ambiguous_glob_reexports, dead_code, unused_imports)]
pub mod prelude {
    pub use {
        crate::export::*,
        crate::reexport::*,
        async_graphql::{extensions::*, *},
        sea_orm::{entity::prelude::*, prelude::*, *},
    };
    pub use {
        async_graphql::{Error as GraphQLErr, MaybeUndefined as Undefined, Schema, Value},
        async_trait::async_trait,
        sea_orm::{
            DbErr, Schema as DbSchema, Value as DbValue,
            sea_query::{IntoCondition, SimpleExpr},
        },
        serde::{Deserialize, Serialize},
        serde_json::Error as JsonErr,
        thiserror::Error as ThisErr,
        tokio::sync::{Mutex, OnceCell},
    };
    _utils::use_common_std!();
}
