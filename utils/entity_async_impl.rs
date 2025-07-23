use crate::*;
use async_graphql::Context;
use async_trait::async_trait;
use sea_orm::prelude::*;
use sea_orm::*;
use std::collections::HashMap;

/// Abstract extra entity async methods implementation.
#[async_trait]
pub trait EntityXAsyncImpl<M, F, O, R>
where
    Self: EntityX<M, F, O, R> + EntityXImpl<M, F, O, R> + 'static,
    M: FromQueryResult + Send + Sync + 'static,
    F: Filter<Self> + 'static,
    O: OrderBy<Self> + 'static,
    R: FromQueryResult + Send + Sync + 'static,
{
    /// Helper to check if exists by condition.
    async fn exists(tx: &DatabaseTransaction, c: Condition) -> Res<bool> {
        let v = Self::find()
            .filter(c)
            .select()
            .expr(Expr::value(1))
            .limit(1)
            .one(tx)
            .await?;
        match v {
            Some(_) => Ok(true),
            _ => Ok(false),
        }
    }
    /// Helper to check if exists by id.
    async fn exists_by_id(tx: &DatabaseTransaction, id: &str) -> Res<bool> {
        Self::exists(tx, Self::by_id(id)).await
    }

    /// Helper to check if exists by condition and return error if not.
    async fn must_exists(tx: &DatabaseTransaction, c: Condition) -> Res<()> {
        match Self::exists(tx, c).await? {
            true => Ok(()),
            false => Err(GrandLineError::Db404),
        }
    }
    /// Helper to check if exists by id and return error if not.
    async fn must_exists_by_id(tx: &DatabaseTransaction, id: &str) -> Res<()> {
        Self::must_exists(tx, Self::by_id(id)).await
    }

    /// Helper to find by id and return error if not.
    async fn must_find_by_id(tx: &DatabaseTransaction, id: &str) -> Res<M> {
        match Self::find().filter(Self::by_id(id)).one(tx).await? {
            Some(v) => Ok(v),
            None => Err(GrandLineError::Db404),
        }
    }

    /// Look ahead for sql columns from requested fields in the graphql context.
    async fn gql_look_ahead(ctx: &Context<'_>) -> Res<Vec<Self::Column>> {
        let k = Self::gql_look_ahead_key(ctx);
        // TODO: cache in the gl context to handle case like: 1000 response nested etc...
        println!("gql_look_ahead k={}", k);

        let f = ctx.look_ahead().selection_fields();
        if f.len() != 1 {
            return Err(GrandLineError::LookAhead);
        }

        let r = f[0]
            .selection_set()
            .filter_map(|f| Self::column(&f.name().to_string()))
            .map(|c| (c.to_string(), c))
            .collect::<HashMap<_, _>>()
            .into_values()
            .collect::<Vec<_>>();
        Ok(r)
    }

    /// Select only columns from requested fields in the graphql context.
    async fn gql_select(ctx: &Context<'_>, q: Select<Self>) -> Res<Selector<SelectModel<R>>> {
        let mut q = q.select_only();
        for c in Self::gql_look_ahead(ctx).await? {
            q = q.select_column(c);
        }
        Ok(q.into_model::<R>())
    }

    /// Helper to use in resolver body of the macro search.
    async fn gql_search(
        ctx: &Context<'_>,
        tx: &DatabaseTransaction,
        condition: Option<Condition>,
        filter: Option<F>,
        filter_extra: Option<F>,
        order_by: Option<Vec<O>>,
        order_by_default: Option<Vec<O>>,
        page: Option<Pagination>,
    ) -> Res<Vec<R>> {
        let mut q = filter.combine(filter_extra).select();
        if let Some(c) = condition {
            q = q.filter(c);
        }
        let q = order_by.combine(order_by_default).chain(q);
        let (limit_default, limit_max) = Self::config_limit();
        let (offset, limit) = page.with(limit_default, limit_max);
        let q = q.offset(offset).limit(limit);
        let r = Self::gql_select(ctx, q).await?.all(tx).await?;
        Ok(r)
    }

    /// Helper to use in resolver body of the macro count.
    async fn gql_count(
        ctx: &Context<'_>,
        tx: &DatabaseTransaction,
        filter: Option<F>,
        filter_extra: Option<F>,
    ) -> Res<u64> {
        let _ = ctx;
        let q = filter.combine(filter_extra).select();
        let r = PaginatorTrait::count(q, tx).await?;
        Ok(r)
    }

    /// Helper to use in resolver body of the macro detail.
    async fn gql_detail(ctx: &Context<'_>, tx: &DatabaseTransaction, id: &str) -> Res<R> {
        let c = Self::by_id(id);
        let q = Self::find().filter(c);
        let r = Self::gql_select(ctx, q).await?.one(tx).await?;
        match r {
            Some(r) => Ok(r),
            None => Err(GrandLineError::Db404),
        }
    }

    /// Helper to use in resolver body of the macro delete.
    async fn gql_delete(ctx: &Context<'_>, tx: &DatabaseTransaction, id: &str) -> Res<R> {
        let c = Self::by_id(id);
        let q = Self::find().filter(c.clone());
        let r = Self::gql_select_id(ctx, q).one(tx).await?;
        match r {
            Some(r) => {
                Self::delete_many().filter(c).exec(tx).await?;
                Ok(r)
            }
            None => Err(GrandLineError::Db404),
        }
    }
}

/// Automatically implement for EntityX.
#[async_trait]
impl<T, M, F, O, R> EntityXAsyncImpl<M, F, O, R> for T
where
    T: EntityX<M, F, O, R> + EntityXImpl<M, F, O, R> + 'static,
    M: FromQueryResult + Send + Sync + 'static,
    F: Filter<T> + 'static,
    O: OrderBy<T> + 'static,
    R: FromQueryResult + Send + Sync + 'static,
{
}
