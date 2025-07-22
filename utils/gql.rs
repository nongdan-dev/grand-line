use crate::*;
use async_graphql::{Context, QueryPathSegment};
use sea_orm::*;
use std::collections::HashMap;

/// Helper trait to abstract gql model methods using sea_orm entity
pub trait Gql<T, F, O, R>
where
    T: EntityTrait,
    F: Filter,
    O: OrderBy,
    R: FromQueryResult,
{
    fn id() -> T::Column;
    /// Helper method to look get sql columns map gql fields
    /// to look ahead and select only requested fields in the async_graphql context
    fn column(field: &str) -> Option<T::Column>;
}

/// Abstract gql model methods implementation
pub trait GqlImpl<T, F, O, R>
where
    T: EntityTrait,
    F: Filter,
    O: OrderBy,
    R: FromQueryResult,
{
    /// Select only columns from requested gql fields in the async_graphql context
    fn look_ahead(ctx: &Context<'_>) -> Vec<T::Column>;
    /// Select only columns from requested gql fields in the async_graphql context
    fn gql_select(ctx: &Context<'_>, q: Select<T>) -> Selector<SelectModel<R>>;
    /// Select only id for the graphql delete response
    fn gql_select_id(q: Select<T>) -> Selector<SelectModel<R>>;
}

impl<T, F, O, R> GqlImpl<T, F, O, R> for T
where
    T: EntityTrait + Gql<T, F, O, R>,
    F: Filter,
    O: OrderBy,
    R: FromQueryResult,
{
    fn look_ahead(ctx: &Context<'_>) -> Vec<T::Column> {
        let k = gql_look_ahead_key(ctx);
        // TODO: cache in the gl context with async_trait
        // TODO: check selection_fields length == 1
        println!("look_ahead k={}", k);

        ctx.look_ahead().selection_fields()[0]
            .selection_set()
            .filter_map(|f| T::column(&f.name().to_string()))
            .map(|c| (c.to_string(), c))
            .collect::<HashMap<_, _>>()
            .into_values()
            .collect::<Vec<_>>()
    }

    fn gql_select(ctx: &Context<'_>, mut q: Select<T>) -> Selector<SelectModel<R>> {
        q = q.select_only();
        for c in T::look_ahead(ctx) {
            q = q.select_column(c);
        }
        q.into_model::<R>()
    }

    fn gql_select_id(q: Select<T>) -> Selector<SelectModel<R>> {
        q.select_only().column(T::id()).into_model::<R>()
    }
}

pub(crate) fn gql_look_ahead_key(ctx: &Context<'_>) -> String {
    let mut arr = vec![];
    let mut next = ctx.path_node;
    while let Some(n) = next {
        if let QueryPathSegment::Name(n) = n.segment {
            arr.push(n.to_string());
        }
        next = n.parent.copied();
    }
    arr.reverse();
    arr.join(".")
}
