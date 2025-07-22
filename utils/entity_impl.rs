use crate::*;
use async_graphql::{Context, QueryPathSegment};
use sea_orm::prelude::*;
use sea_orm::*;

/// Abstract extra entity methods implementation.
pub trait EntityXImpl<M, F, O, R>
where
    Self: EntityX<M, F, O, R>,
    M: FromQueryResult + Send + Sync,
    F: Filter<Self>,
    O: OrderBy<Self>,
    R: FromQueryResult + Send + Sync,
{
    /// Convert primary id string into condition to use in abstract methods.
    fn by_id(id: &str) -> Condition {
        Condition::all().add(Self::id().eq(id))
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

    /// Select only id for the graphql delete response.
    fn gql_select_id(ctx: &Context<'_>, q: Select<Self>) -> Selector<SelectModel<R>> {
        let _ = ctx;
        q.select_only().column(Self::id()).into_model::<R>()
    }
}

/// Automatically implement for EntityX.
impl<T, M, F, O, R> EntityXImpl<M, F, O, R> for T
where
    T: EntityX<M, F, O, R>,
    M: FromQueryResult + Send + Sync,
    F: Filter<T>,
    O: OrderBy<T>,
    R: FromQueryResult + Send + Sync,
{
}
