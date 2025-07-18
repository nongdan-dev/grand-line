pub use grand_line_macros::{order_by, order_by_some};
pub use grand_line_proc_macros::{
    active_create, active_model, active_update, count, create, delete, detail, filter, filter_some,
    input, model, mutation, query, search, update,
};

pub use async_graphql;
pub use chrono;
pub use sea_orm;
pub use serde;
pub use serde_json;
pub use serde_with;
pub use sqlx;
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

pub mod grand_line_macros {
    pub use grand_line_macros::*;
    pub use grand_line_proc_macros::*;
    pub use paste::paste;
    pub use proc_macro2::TokenStream as TokenStream2;
    pub use quote::quote;
}

#[input]
pub struct Pagination {
    pub limit: Option<u64>,
    pub offset: Option<u64>,
}

use sea_orm::entity::prelude::*;
use sea_orm::*;

pub trait Conditionable {
    fn condition(&self) -> Condition;
}

pub trait Chainable<T>
where
    T: EntityTrait,
{
    fn chain(&self, q: Select<T>) -> Select<T>;
}
impl<T, F> Chainable<T> for Option<F>
where
    T: EntityTrait,
    F: Chainable<T>,
{
    fn chain(&self, q: Select<T>) -> Select<T> {
        match self {
            Some(c) => c.chain(q),
            None => q,
        }
    }
}
impl<T, F> Chainable<T> for Vec<F>
where
    T: EntityTrait,
    F: Chainable<T>,
{
    fn chain(&self, q: Select<T>) -> Select<T> {
        let mut q = q;
        for c in self {
            q = c.chain(q)
        }
        q
    }
}

pub trait Queryable<E: EntityTrait> {
    fn query(&self) -> Select<E>;
}
impl<T, E> Queryable<E> for T
where
    T: Chainable<E>,
    E: EntityTrait,
{
    fn query(&self) -> Select<E> {
        self.chain(E::find())
    }
}

pub struct CrudHelpers;

impl CrudHelpers {
    /**
     * Helper to combine filter and extra_filter
     */
    pub fn filter_combine<T>(a: Option<T>, b: Option<T>, v: &dyn Fn(T, T) -> T) -> Option<T> {
        match (a, b) {
            (Some(a), Some(b)) => Some(v(a, b)),
            (Some(a), None) => Some(a),
            (None, Some(b)) => Some(b),
            (None, None) => None,
        }
    }
    /**
     * Helper to combine order_by and default_order_by with an initial value if all are empty
     */
    pub fn order_by_combine<T>(a: Option<Vec<T>>, b: Option<Vec<T>>, v: T) -> Vec<T> {
        match a {
            Some(a) => match a.len() {
                0 => Self::default_order_by_opt(b, v),
                _ => a,
            },
            None => Self::default_order_by_opt(b, v),
        }
    }
    /**
     * Helper to combine order_by and default_order_by with an initial value if all are empty
     */
    pub fn default_order_by_opt<T>(a: Option<Vec<T>>, v: T) -> Vec<T> {
        match a {
            Some(a) => Self::default_order_by_vec(a, v),
            None => vec![v],
        }
    }
    /**
     * Helper to combine order_by and default_order_by with an initial value if all are empty
     */
    pub fn default_order_by_vec<T>(a: Vec<T>, v: T) -> Vec<T> {
        match a.len() {
            0 => vec![v],
            _ => a,
        }
    }
    /**
     * Helper to get pagination
     */
    pub fn pagination(p: Option<Pagination>, default_limit: u64, max_limit: u64) -> (u64, u64) {
        match p {
            Some(p) => Self::default_pagination(p, default_limit, max_limit),
            None => (0, default_limit),
        }
    }
    /**
     * Helper to get pagination
     */
    pub fn default_pagination(p: Pagination, default_limit: u64, max_limit: u64) -> (u64, u64) {
        (
            if let Some(o) = p.offset { o } else { 0 },
            if let Some(l) = p.limit {
                if l > max_limit {
                    max_limit
                } else {
                    l
                }
            } else {
                default_limit
            },
        )
    }
}
