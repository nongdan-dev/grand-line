use crate::prelude::*;
use async_graphql::Context;

/// Abstract extra entity async methods implementation.
#[async_trait]
pub trait EntityXAsyncImpl<M, A, F, O, G>
where
    Self: EntityX<M, A, F, O, G> + 'static,
    M: FromQueryResult + Send + Sync + 'static,
    A: ActiveModelTrait<Entity = Self> + 'static,
    F: Filter<Self> + 'static,
    O: OrderBy<Self> + 'static,
    G: FromQueryResult + Send + Sync + 'static,
{
    /// Helper to check if exists by condition.
    async fn exists<D>(db: &D, c: Condition) -> Res<bool>
    where
        D: ConnectionTrait,
    {
        let v = Self::find()
            .filter(c)
            .select()
            .expr(Expr::value(1))
            .limit(1)
            .one(db)
            .await?;
        match v {
            Some(_) => Ok(true),
            None => Ok(false),
        }
    }
    /// Helper to check if exists by id.
    async fn exists_by_id<D>(db: &D, id: &str) -> Res<bool>
    where
        D: ConnectionTrait,
    {
        Self::exists(db, Self::condition_id(id)).await
    }

    /// Helper to check if exists by condition and return error if not.
    async fn must_exists<D>(db: &D, c: Condition) -> Res<()>
    where
        D: ConnectionTrait,
    {
        match Self::exists(db, c).await? {
            true => Ok(()),
            false => err_client!(Db404),
        }
    }
    /// Helper to check if exists by id and return error if not.
    async fn must_exists_by_id<D>(db: &D, id: &str) -> Res<()>
    where
        D: ConnectionTrait,
    {
        Self::must_exists(db, Self::condition_id(id)).await
    }

    /// Helper to find by id and return error if not.
    async fn must_find_by_id<D>(db: &D, id: &str) -> Res<M>
    where
        D: ConnectionTrait,
    {
        match Self::find().filter(Self::condition_id(id)).one(db).await? {
            Some(v) => Ok(v),
            None => err_client!(Db404),
        }
    }

    /// Look ahead for sql columns from requested fields in the graphql context.
    async fn gql_look_ahead(
        ctx: &Context<'_>,
    ) -> Res<
        Vec<(
            Option<Self::Column>,
            Option<(&'static str, sea_query::SimpleExpr)>,
        )>,
    > {
        let k = Self::gql_look_ahead_key(ctx);
        // TODO: cache in the gl context to handle case like: 1000 response nested etc...
        println!("gql_look_ahead k={}", k);

        let f = ctx.look_ahead().selection_fields();
        if f.len() != 1 {
            return err_server!(LookAhead);
        }

        let r = f[0]
            .selection_set()
            .filter_map(|f| {
                let name = f.name().to_string();
                match Self::config_gql_select(&name) {
                    (None, None) => None,
                    (o1, o2) => Some((name, o1, o2)),
                }
            })
            .map(|(f, o1, o2)| (f, (o1, o2)))
            .collect::<HashMap<_, _>>()
            .into_values()
            .collect::<Vec<_>>();

        Ok(r)
    }

    /// Helper to use in resolver body of the macro search.
    async fn gql_search<D>(
        ctx: &Context<'_>,
        db: &D,
        condition: Option<Condition>,
        filter: Option<F>,
        filter_extra: Option<F>,
        order_by: Option<Vec<O>>,
        order_by_default: Option<Vec<O>>,
        page: Option<Pagination>,
    ) -> Res<Vec<G>>
    where
        D: ConnectionTrait,
    {
        let mut q = filter.combine(filter_extra).select();
        if let Some(c) = condition {
            q = q.filter(c);
        }
        let q = order_by.combine(order_by_default).chain(q);
        let (limit_default, limit_max) = Self::config_limit();
        let (offset, limit) = page.with(limit_default, limit_max);
        let r = q
            .offset(offset)
            .limit(limit)
            .gql_select(ctx)
            .await?
            .all(db)
            .await?;
        Ok(r)
    }

    /// Helper to use in resolver body of the macro count.
    async fn gql_count<D>(db: &D, filter: Option<F>, filter_extra: Option<F>) -> Res<u64>
    where
        D: ConnectionTrait,
    {
        let q = filter.combine(filter_extra).select();
        let r = PaginatorTrait::count(q, db).await?;
        Ok(r)
    }

    /// Helper to use in resolver body of the macro detail.
    async fn gql_detail<D>(ctx: &Context<'_>, db: &D, id: &str) -> Res<Option<G>>
    where
        D: ConnectionTrait,
    {
        let r = Self::internal_find_by_id(id)
            .gql_select(ctx)
            .await?
            .one(db)
            .await?;
        Ok(r)
    }

    /// Helper to use in resolver body of the macro delete.
    async fn gql_delete<D>(db: &D, id: &str) -> Res<G>
    where
        D: ConnectionTrait,
    {
        let c = Self::condition_id(id);
        let r = Self::find()
            .filter(c.clone())
            .gql_select_id()
            .one(db)
            .await?;
        match r {
            Some(r) => {
                Self::delete_many().filter(c).exec(db).await?;
                Ok(r)
            }
            None => err_client!(Db404),
        }
    }
}

/// Automatically implement for EntityX.
#[async_trait]
impl<T, M, A, F, O, G> EntityXAsyncImpl<M, A, F, O, G> for T
where
    T: EntityX<M, A, F, O, G> + 'static,
    M: FromQueryResult + Send + Sync + 'static,
    A: ActiveModelTrait<Entity = T> + 'static,
    M: FromQueryResult + Send + Sync + 'static,
    F: Filter<T> + 'static,
    O: OrderBy<T> + 'static,
    G: FromQueryResult + Send + Sync + 'static,
{
}
