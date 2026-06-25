use super::prelude::*;

/// Helper trait to abstract extra methods into `sea_orm` entity.
#[async_trait]
pub trait EntityX
where
    Self: EntityTrait<Model = Self::M, ActiveModel = Self::A, Column = Self::C>,
{
    type M: ModelX<Self>;
    type A: ActiveModelX<Self>;
    type C: ColumnX<Self>;
    type F: Filter<Self>;
    type O: OrderBy<Self>;
    type G: GqlModel<Self>;

    /// Get entity model name.
    /// To clarify model name in case of error.
    fn model_name() -> &'static str;

    /// Get column id.
    /// Should be generated in the model macro.
    fn col_id() -> Self::C;
    /// Get column `created_at`.
    /// Should be generated in the model macro.
    fn col_created_at() -> Option<Self::C>;
    /// Get column `updated_at`.
    /// Should be generated in the model macro.
    fn col_updated_at() -> Option<Self::C>;
    /// Get column `deleted_at`.
    /// Should be generated in the model macro.
    fn col_deleted_at() -> Option<Self::C>;
    /// Get column `created_by_id`.
    /// Should be generated in the model macro.
    fn col_created_by_id() -> Option<Self::C>;
    /// Get column `updated_by_id`.
    /// Should be generated in the model macro.
    fn col_updated_by_id() -> Option<Self::C>;
    /// Get column `deleted_by_id`.
    /// Should be generated in the model macro.
    fn col_deleted_by_id() -> Option<Self::C>;

    /// Get sql columns map with rust snake field name, for gql look ahead.
    /// Exclude all columns skipped with #[graphql(skip)].
    /// Should be generated in the model macro.
    fn gql_cols() -> &'static LazyLock<HashMap<&'static str, Self::C>>;
    /// Get sql exprs map with rust snake field name, for gql look ahead.
    /// Should be generated in the model macro.
    fn gql_exprs() -> &'static LazyLock<HashMap<&'static str, SimpleExpr>>;
    /// Get rust snake field name sql columns, from gql camel field, for gql look ahead.
    /// To look ahead and select only requested fields in the gql context.
    /// Should be generated in the model macro.
    fn gql_select() -> &'static LazyLock<HashMap<&'static str, HashSet<&'static str>>>;

    /// Look ahead for sql columns and exprs, from requested fields in the gql context.
    fn gql_look_ahead(ctx: &Context<'_>) -> Res<Vec<LookaheadX<Self>>> {
        let f = ctx.look_ahead().selection_fields();

        let gql_cols = Self::gql_cols();
        let gql_exprs = Self::gql_exprs();
        let gql_select = Self::gql_select();

        let r = f
            .first()
            .ok_or(MyErr::GqlLookAhead)?
            .selection_set()
            .filter_map(|f| gql_select.get(f.name().to_owned().as_str()))
            .flat_map(|c| c.iter().copied())
            .collect::<HashSet<_>>()
            .iter()
            .filter_map(|c| {
                let (col, expr) = (gql_cols.get(c), gql_exprs.get(c));
                match (col, expr) {
                    (None, None) => None,
                    _ => Some(LookaheadX {
                        c,
                        col: col.copied(),
                        expr: expr.cloned(),
                    }),
                }
            })
            .collect::<Vec<_>>();

        Ok(r)
    }

    /// Quickly build condition id eq.
    fn cond_id(id: &str) -> Condition {
        Condition::all().add(Self::col_id().eq(id))
    }

    /// Ensure `deleted_at` column is present.
    fn ensure_col_deleted_at() -> Res<Self::C> {
        let col = Self::col_deleted_at().ok_or_else(|| MyErr::DbCol404 {
            col: Self::model_name().to_owned() + ".deleted_at",
        })?;
        Ok(col)
    }
    /// Quickly build condition exclude deleted.
    fn cond_exclude_deleted() -> Option<Condition> {
        Self::col_deleted_at().map(|c| Condition::all().add(c.is_null()))
    }

    /// Set `deleted_at` with filter by id.
    /// It also checks if the model has configured with `deleted_at` column or not.
    fn soft_delete_by_id(id: &str) -> Res<UpdateMany<Self>> {
        let r = Self::soft_delete_many()?.filter_by_id(id);
        Ok(r)
    }

    /// Set `deleted_at` without any filter.
    /// It also checks if the model has configured with `deleted_at` column or not.
    fn soft_delete_many() -> Res<UpdateMany<Self>> {
        Self::ensure_col_deleted_at()?;
        let am = Self::A::defaults_on_delete();
        let r = Self::update_many().set(am);
        Ok(r)
    }

    /// Helper to use in resolver body of the macro search.
    async fn gql_search<D>(
        ctx: &Context<'_>,
        tx: &D,
        // From graphql input.
        filter: Option<Self::F>,
        order_by: Option<Vec<Self::O>>,
        page: Option<Pagination>,
        include_deleted: Option<bool>,
        // From resolver handler.
        filter_extra: Option<Self::F>,
        order_by_default: Option<Vec<Self::O>>,
        // From relation.
        extra_cond: Option<Condition>,
        // From macro to handle authz row filter.
        authz_row_filter: Option<Self::F>,
    ) -> Res<Vec<Self::G>>
    where
        D: ConnectionTrait,
    {
        // should not combine authz role filter, it will change the exclude_deleted condition
        let f = filter.combine(filter_extra);
        let exclude_deleted = !include_deleted.or_else(|| Some(f.has_deleted_at())).unwrap_or_default();
        let mut r = Self::find();
        if exclude_deleted {
            r = r.exclude_deleted();
        }
        let r = r
            .filter_opt(extra_cond)
            .chain(f)
            .chain(authz_row_filter)
            .chain(order_by.combine(order_by_default))
            .chain(page.inner(ctx.core_config()))
            .gql_select(ctx)?
            .all(tx)
            .await?;
        Ok(r)
    }

    /// Helper to use in resolver body of the macro count.
    async fn gql_count<D>(
        tx: &D,
        // From graphql input.
        filter: Option<Self::F>,
        include_deleted: Option<bool>,
        // From resolver handler.
        filter_extra: Option<Self::F>,
        // From macro to handle authz row filter.
        authz_row_filter: Option<Self::F>,
    ) -> Res<u64>
    where
        D: ConnectionTrait,
    {
        // should not combine authz role filter, it will change the exclude_deleted condition
        let f = filter.combine(filter_extra);
        let exclude_deleted = !include_deleted.or_else(|| Some(f.has_deleted_at())).unwrap_or_default();
        let mut r = Self::find();
        if exclude_deleted {
            r = r.exclude_deleted();
        }
        let r = r.chain(f).chain(authz_row_filter).count(tx).await?;
        Ok(r)
    }

    /// Helper to use in resolver body of the macro detail.
    async fn gql_detail<D>(
        ctx: &Context<'_>,
        tx: &D,
        id: &str,
        include_deleted: Option<bool>,
        authz_row_filter: Option<Self::F>,
    ) -> Res<Option<Self::G>>
    where
        D: ConnectionTrait,
    {
        // should not combine authz role filter, it will change the exclude_deleted condition
        let exclude_deleted = !include_deleted.unwrap_or_default();
        let mut q = Self::find();
        if exclude_deleted {
            q = q.exclude_deleted();
        }
        let r = q
            .filter_by_id(id)
            .chain(authz_row_filter)
            .gql_select(ctx)?
            .one(tx)
            .await?;
        Ok(r)
    }

    /// Helper to use in resolver body of the macro update/delete.
    async fn gql_mutation_check_id<D>(
        tx: &D,
        id: &str,
        authz_row_filter: Option<Self::F>,
        authz_err: &GrandLineErr,
    ) -> Res<()>
    where
        D: ConnectionTrait,
    {
        let q = || Self::find().filter_by_id(id);

        let Some(f) = authz_row_filter else {
            q().exists_or_404(tx).await?;
            return Ok(());
        };

        if !q().chain(f).exists(tx).await? {
            return Err(authz_err.clone());
        }

        Ok(())
    }

    /// Helper to use in resolver body of the macro update.
    async fn gql_update<D>(
        tx: &D,
        id: &str,
        am: Self::A,
        authz_row_filter: Option<Self::F>,
        authz_err: &GrandLineErr,
    ) -> Res<Self::G>
    where
        D: ConnectionTrait,
    {
        let mut q = Self::update_many().filter_by_id(id);
        if let Some(f) = authz_row_filter {
            q = q.filter(f.into_condition());
        }
        let rows_affected = q.set(am).exec(tx).await?.rows_affected;

        if rows_affected == 0 {
            return Err(authz_err.clone());
        }

        let r = Self::G::from_id(id);
        Ok(r)
    }

    /// Helper to use in resolver body of the macro delete.
    async fn gql_delete<D>(
        tx: &D,
        id: &str,
        permanent: Option<bool>,
        authz_row_filter: Option<Self::F>,
        authz_err: &GrandLineErr,
    ) -> Res<Self::G>
    where
        D: ConnectionTrait,
    {
        let rows_affected = if permanent.unwrap_or_default() {
            let mut q = Self::delete_many().filter_by_id(id);
            if let Some(f) = authz_row_filter {
                q = q.filter(f.into_condition());
            }
            q.exec(tx).await?.rows_affected
        } else {
            let mut q = Self::soft_delete_by_id(id)?;
            if let Some(f) = authz_row_filter {
                q = q.filter(f.into_condition());
            }
            q.exec(tx).await?.rows_affected
        };

        if rows_affected == 0 {
            return Err(authz_err.clone());
        }

        let r = Self::G::from_id(id);
        Ok(r)
    }

    async fn gql_load<D>(
        ctx: &Context<'_>,
        _tx: &D,
        col: Self::C,
        id: String,
        authz_row_filter: Option<Self::F>,
        include_deleted: Option<bool>,
    ) -> Res<Option<Self::G>>
    where
        D: ConnectionTrait,
    {
        let look_ahead = Self::gql_look_ahead(ctx)?;
        let exclude_deleted = !include_deleted.unwrap_or_default();
        let exclude_deleted = if exclude_deleted {
            Self::cond_exclude_deleted()
        } else {
            None
        };
        let authz_key = authz_row_filter.as_ref().map(json_string).transpose()?;
        let authz_condition = authz_row_filter.map(|f| f.into_condition());
        let key = col.to_loader_key(&look_ahead, exclude_deleted.is_some(), authz_key.as_deref());
        ctx.data_loader(key, col, look_ahead, exclude_deleted, authz_condition)
            .await?
            .as_ref()
            .load_one(id)
            .await
    }
}
