use super::prelude::*;

/// Abstract extra entity async methods implementation.
#[async_trait]
pub trait EntityXAsync
where
    Self: EntityX,
{
    /// Helper to use in resolver body of the macro search.
    async fn gql_search<D>(
        ctx: &Context<'_>,
        db: &D,
        extra_cond: Option<Condition>,
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
            .filter_optional(extra_cond)
            .chain(f)
            .chain(order_by.combine(order_by_default))
            .chain(page.inner(ctx.config()))
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
        let f = filter.combine(filter_extra);
        let include_deleted = include_deleted.or_else(|| Some(f.has_deleted_at()));
        let r = Self::find()
            .include_deleted(include_deleted)
            .chain(f)
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
            Self::soft_delete_by_id(id)?.exec(db).await?;
        }
        let r = Self::G::default()._set_id(id);
        Ok(r)
    }

    async fn gql_load(
        ctx: &Context<'_>,
        col: Self::C,
        id: String,
        include_deleted: Option<bool>,
    ) -> Res<Option<Self::G>> {
        let look_ahead = Self::gql_look_ahead(ctx)?;
        let include_deleted = Self::_cond_deleted_at(include_deleted);
        let key = col.build_loader_key(&look_ahead, include_deleted.is_some());
        ctx.data_loader(key, col, look_ahead, include_deleted)
            .await?
            .as_ref()
            .load_one(id)
            .await
    }
}

/// Automatically implement for EntityX.
#[async_trait]
impl<E> EntityXAsync for E where E: EntityX {}
