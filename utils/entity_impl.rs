use crate::*;
use async_graphql::{Context, QueryPathSegment};
use sea_orm::prelude::*;
use sea_orm::*;

/// Abstract extra entity methods implementation.
pub trait EntityXImpl<M, A, F, O, G>
where
    Self: EntityX<M, A, F, O, G>,
    M: FromQueryResult + Send + Sync,
    A: ActiveModelTrait<Entity = Self>,
    F: Filter<Self>,
    O: OrderBy<Self>,
    G: FromQueryResult + Send + Sync,
{
    /// Shortcut condition id eq.
    fn condition_id(id: &str) -> Condition {
        Condition::all().add(Self::config_col_id().eq(id))
    }
    /// Shortcut condition deleted_at is not null, if there is deleted_at.
    fn condition_exclude_deleted() -> Condition {
        let mut c = Condition::all();
        if let Some(col) = Self::config_col_deleted_at() {
            c = c.add(col.is_null())
        }
        c
    }

    /// Get look ahead key with alias-aware.
    /// For example the below two queries both have the same key a.b at b if there is no alias-aware.
    /// But the selection sets are different so they need to have different keys:
    /// query q1 {
    ///   a {
    ///     b {
    ///       c
    ///     }
    ///   }
    /// }
    /// query q2 {
    ///   x: a {
    ///     b {
    ///       c
    ///       d
    ///     }
    ///   }
    /// }
    fn gql_look_ahead_key(ctx: &Context<'_>) -> String {
        let mut arr = vec![];
        let mut next = ctx.path_node;
        // TODO: alias
        while let Some(n) = next {
            if let QueryPathSegment::Name(n) = n.segment {
                arr.push(n.to_string());
            }
            next = n.parent.copied();
        }
        arr.reverse();
        arr.join(".")
    }
}

/// Automatically implement for EntityX.
impl<T, M, A, F, O, G> EntityXImpl<M, A, F, O, G> for T
where
    T: EntityX<M, A, F, O, G>,
    M: FromQueryResult + Send + Sync,
    A: ActiveModelTrait<Entity = T>,
    F: Filter<T>,
    O: OrderBy<T>,
    G: FromQueryResult + Send + Sync,
{
}

/// Abstract extra entity internal methods implementation.
pub(crate) trait EntityXInternalImpl<M, A, F, O, G>
where
    Self: EntityX<M, A, F, O, G>,
    A: ActiveModelTrait<Entity = Self>,
    M: FromQueryResult + Send + Sync,
    F: Filter<Self>,
    O: OrderBy<Self>,
    G: FromQueryResult + Send + Sync,
{
    fn internal_find_by_id(id: &str) -> Select<Self> {
        Self::find().filter(Self::condition_id(id))
    }
}

/// Automatically implement for EntityX.
impl<T, M, A, F, O, G> EntityXInternalImpl<M, A, F, O, G> for T
where
    T: EntityX<M, A, F, O, G>,
    A: ActiveModelTrait<Entity = T>,
    M: FromQueryResult + Send + Sync,
    F: Filter<T>,
    O: OrderBy<T>,
    G: FromQueryResult + Send + Sync,
{
}
