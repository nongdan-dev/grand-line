use crate::prelude::*;

/// Helper trait to abstract extra methods into sea_orm entity.
pub trait EntityX: EntityTrait<Model = Self::M> {
    type M: ModelX<Self>;
    type A: ActiveModelX<Self>;
    type F: Filter<Self>;
    type O: OrderBy<Self>;
    type G: GqlModel<Self>;

    /// Get default and max limit configuration.
    /// Should be generated in the model macro.
    fn _limit_config() -> LimitConfig;

    /// Get sql columns map with rust snake field name to use in abstract methods.
    /// Should be generated in the model macro.
    fn _sql_cols() -> &'static LazyLock<HashMap<&'static str, Self::Column>>;

    /// Get sql exprs map with rust snake field name to use in abstract methods.
    /// Should be generated in the model macro.
    fn _sql_exprs() -> &'static LazyLock<HashMap<&'static str, sea_query::SimpleExpr>>;

    /// Get rust snake field sql columns and exprs, from gql camel field.
    /// To look ahead and select only requested fields in the gql context.
    /// Should be generated in the model macro.
    fn _gql_select() -> &'static LazyLock<HashMap<&'static str, Vec<&'static str>>>;

    /// Look ahead for sql columns and exprs, from requested fields in the gql context.
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

        let sql_cols = Self::_sql_cols();
        let sql_exprs = Self::_sql_exprs();
        let gql_select = Self::_gql_select();

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

    /// Get primary id column to use in abstract methods.
    fn _col_id() -> Res<Self::Column> {
        Self::_sql_cols()
            .get("id")
            .cloned()
            .ok_or_else(|| ErrServer::BugId404.into())
    }

    /// Shortcut condition id eq.
    fn _cond_id(id: &str) -> Res<Condition> {
        Self::_col_id().map(|c| Condition::all().add(c.eq(id)))
    }

    /// Get deleted at column to use in abstract methods.
    fn _col_deleted_at() -> Option<Self::Column> {
        Self::_sql_cols().get("deleted_at").cloned()
    }

    /// Shortcut condition include deleted.
    fn _cond_deleted_at(include_deleted: Option<bool>) -> Option<Condition> {
        match include_deleted {
            Some(true) => None,
            _ => match Self::_col_deleted_at() {
                Some(c) => Some(Condition::all().add(c.is_null())),
                None => None,
            },
        }
    }

    fn soft_delete_by_id(id: &str) -> Self::A {
        <Self::A as Default>::default()._set_id(id)._delete()
    }
}
