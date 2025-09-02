use crate::prelude::*;

/// Helper trait to abstract extra methods into sea_orm entity.
pub trait EntityX: EntityTrait<Model = Self::M> {
    type M: ModelX<Self> + Send + Sync;
    type A: ActiveModelX<Self> + Send + Sync;
    type F: Filter<Self> + Send + Sync;
    type O: OrderBy<Self> + Send + Sync;
    type G: GqlModel<Self> + Send + Sync;

    /// Get default and max limit configuration.
    /// Should be generated in the #[model] macro.
    fn conf_limit() -> ConfigLimit;

    /// sea_orm ActiveModel hooks will not be called with Entity:: or bulk methods.
    /// We need to have this method instead to get default values on create.
    /// This can be used together with the macro grand_line::am_create.
    /// Should be generated in the #[model] macro.
    fn conf_am_create(am: Self::A) -> Self::A;

    /// sea_orm ActiveModel hooks will not be called with Entity:: or bulk methods.
    /// We need to have this method instead to get default values on update.
    /// This can be used together with the macro grand_line::am_update.
    /// Should be generated in the #[model] macro.
    fn conf_am_update(am: Self::A) -> Self::A;

    /// sea_orm ActiveModel hooks will not be called with Entity:: or bulk methods.
    /// We need to have this method instead to get default values on delete.
    /// This can be used together with the macro grand_line::am_delete.
    /// Should be generated in the #[model] macro.
    fn conf_am_soft_delete(am: Self::A) -> Self::A;

    /// Get sql columns map with rust snake field name to use in abstract methods.
    /// Should be generated in the #[model] macro.
    fn conf_sql_cols() -> &'static LazyLock<HashMap<&'static str, Self::Column>>;

    /// Get sql exprs map with rust snake field name to use in abstract methods.
    /// Should be generated in the #[model] macro.
    fn conf_sql_exprs() -> &'static LazyLock<HashMap<&'static str, sea_query::SimpleExpr>>;

    /// Get sql columns and exprs from gql field to look ahead
    /// to select only requested fields in the graphql context.
    /// Should be generated in the #[model] macro.
    fn conf_gql_select() -> &'static LazyLock<HashMap<&'static str, Vec<&'static str>>>;

    /// Get primary id column to use in abstract methods.
    fn conf_col_id() -> Res<Self::Column> {
        Self::conf_sql_cols()
            .get("id")
            .cloned()
            .ok_or_else(|| ErrServer::BugId404.into())
    }

    /// Shortcut condition id eq.
    fn cond_id(id: &str) -> Res<Condition> {
        Self::conf_col_id().map(|c| Condition::all().add(c.eq(id)))
    }

    /// Get deleted at column to use in abstract methods.
    fn conf_col_deleted_at() -> Option<Self::Column> {
        Self::conf_sql_cols().get("deleted_at").cloned()
    }

    /// Shortcut condition include deleted.
    fn cond_include_deleted(include_deleted: Option<bool>) -> Option<Condition> {
        match include_deleted {
            Some(true) => None,
            _ => match Self::conf_col_deleted_at() {
                Some(c) => Some(Condition::all().add(c.is_null())),
                None => None,
            },
        }
    }

    /// Look ahead for sql columns from requested fields in the graphql context.
    fn gql_look_ahead(
        ctx: &Context<'_>,
    ) -> Res<
        Vec<(
            &'static str,
            Option<Self::Column>,
            Option<sea_query::SimpleExpr>,
        )>,
    > {
        let f = ctx.look_ahead().selection_fields();
        if f.len() != 1 {
            return err_server!(LookAhead);
        }

        let sql_cols = Self::conf_sql_cols();
        let sql_exprs = Self::conf_sql_exprs();
        let gql_select = Self::conf_gql_select();

        let r = f[0]
            .selection_set()
            .filter_map(|f| gql_select.get(f.name().to_string().as_str()))
            .flat_map(|c| c.iter().copied())
            .collect::<HashSet<_>>()
            .iter()
            .filter_map(|c| {
                let (col, expr) = (sql_cols.get(c), sql_exprs.get(c));
                match (col, expr) {
                    (None, None) => None,
                    _ => Some((*c, col.copied(), expr.cloned())),
                }
            })
            .collect::<Vec<_>>();

        Ok(r)
    }
}
