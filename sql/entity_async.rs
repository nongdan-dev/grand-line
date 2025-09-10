use crate::prelude::*;

/// Abstract extra entity async methods implementation.
#[async_trait]
pub trait EntityXAsync: EntityX {
    /// Helper to use in resolver body of the macro search.
    async fn gql_search<D>(
        ctx: &Context<'_>,
        db: &D,
        condition: Option<Condition>,
        filter: Option<Self::F>,
        filter_extra: Option<Self::F>,
        order_by: Option<Vec<Self::O>>,
        order_by_default: Option<Vec<Self::O>>,
        page: Option<Pagination>,
        include_deleted: Option<bool>,
    ) -> Res<Vec<Self::G>>
    where
        D: ConnectionTrait,
    {
        let f = filter.combine(filter_extra);
        let r = Self::find()
            .include_deleted(include_deleted.or_else(|| Some(f.has_deleted_at())))
            .filter_opt(condition)
            .chain(f)
            .chain(order_by.combine(order_by_default))
            .chain(page.inner(Self::_limit_config()))
            .gql_select(ctx)?
            .all(db)
            .await?;
        Ok(r)
    }

    /// Helper to use in resolver body of the macro count.
    async fn gql_count<D>(
        db: &D,
        filter: Option<Self::F>,
        filter_extra: Option<Self::F>,
        include_deleted: Option<bool>,
    ) -> Res<u64>
    where
        D: ConnectionTrait,
    {
        let filter = filter.combine(filter_extra);
        let include_deleted = include_deleted.or_else(|| Some(filter.has_deleted_at()));
        let r = Self::find()
            .include_deleted(include_deleted)
            .filter_opt(filter.map(|f| f.cond()))
            .count(db)
            .await?;
        Ok(r)
    }

    /// Helper to use in resolver body of the macro detail.
    async fn gql_detail<D>(
        ctx: &Context<'_>,
        db: &D,
        id: &str,
        include_deleted: Option<bool>,
    ) -> Res<Option<Self::G>>
    where
        D: ConnectionTrait,
    {
        let r = Self::find()
            .include_deleted(include_deleted)
            .by_id(id)?
            .gql_select(ctx)?
            .one(db)
            .await?;
        Ok(r)
    }

    /// Helper to use in resolver body of the macro delete.
    async fn gql_delete<D>(db: &D, id: &str, permanent: Option<bool>) -> Res<Self::G>
    where
        D: ConnectionTrait,
    {
        if permanent.unwrap_or_default() {
            Self::delete_many().by_id(id)?.exec(db).await?;
        } else {
            let a = <Self::A as Default>::default();
            a._set_id(id)._delete().update(db).await?;
        }
        let r = Self::G::default()._set_id(id);
        Ok(r)
    }
}

/// Automatically implement for EntityX.
#[async_trait]
impl<T> EntityXAsync for T where T: EntityX {}
