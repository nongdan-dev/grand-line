use super::prelude::*;

/// Helper trait to abstract extra methods into sea_orm entity.
pub trait EntityX: EntityTrait<Model = Self::M, ActiveModel = Self::A, Column = Self::C> {
    type M: ModelX<Self>;
    type A: ActiveModelX<Self>;
    type C: ColumnX<Self>;
    type F: Filter<Self>;
    type O: OrderBy<Self>;
    type G: GqlModel<Self>;

    /// Get entity model name.
    /// To clarify model name in case of error.
    fn _model_name() -> &'static str;

    /// Get sql columns map with rust snake field name.
    /// Should be generated in the model macro.
    fn _gql_cols() -> &'static LazyLock<HashMap<&'static str, Self::C>>;

    /// Get sql exprs map with rust snake field name.
    /// Should be generated in the model macro.
    fn _gql_exprs() -> &'static LazyLock<HashMap<&'static str, SimpleExpr>>;

    /// Get rust snake field name sql columns, from gql camel field.
    /// To look ahead and select only requested fields in the gql context.
    /// Should be generated in the model macro.
    fn _gql_select() -> &'static LazyLock<HashMap<&'static str, HashSet<&'static str>>>;

    /// Look ahead for sql columns and exprs, from requested fields in the gql context.
    fn gql_look_ahead(ctx: &Context<'_>) -> Res<Vec<LookaheadX<Self>>> {
        let f = ctx.look_ahead().selection_fields();
        if f.len() != 1 {
            err!(LookAhead)?;
        }

        let gql_cols = Self::_gql_cols();
        let gql_exprs = Self::_gql_exprs();
        let gql_select = Self::_gql_select();

        let r = f[0]
            .selection_set()
            .filter_map(|f| gql_select.get(f.name().to_string().as_str()))
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

    /// Get primary id column.
    fn _col_id() -> Res<Self::C> {
        let col = Self::_gql_cols()
            .get("id")
            .cloned()
            .ok_or_else(|| MyErr::BugId404 {
                model: Self::_model_name(),
            })?;
        Ok(col)
    }
    /// Quickly build condition id eq.
    fn _cond_id(id: &str) -> Res<Condition> {
        Self::_col_id().map(|c| Condition::all().add(c.eq(id)))
    }

    /// Get created_at column.
    fn _col_created_at() -> Option<Self::C> {
        Self::_gql_cols().get("created_at").cloned()
    }
    /// Get updated_at column.
    fn _col_updated_at() -> Option<Self::C> {
        Self::_gql_cols().get("updated_at").cloned()
    }

    /// Get deleted_at column.
    fn _col_deleted_at() -> Option<Self::C> {
        Self::_gql_cols().get("deleted_at").cloned()
    }
    /// Get deleted_at column.
    fn _check_col_deleted_at() -> Res<Self::C> {
        let col = Self::_col_deleted_at().ok_or_else(|| MyErr::DbCfgField404 {
            model: Self::_model_name(),
            field: "deleted_at",
        })?;
        Ok(col)
    }
    /// Quickly build condition include deleted.
    fn _cond_deleted_at(include_deleted: Option<bool>) -> Option<Condition> {
        match include_deleted {
            Some(true) => None,
            _ => Self::_col_deleted_at().map(|c| Condition::all().add(c.is_null())),
        }
    }

    /// Set deleted_at with filter by id.
    /// It also checks if the model has configured with deleted_at column or not.
    fn soft_delete_by_id(id: &str) -> Res<UpdateMany<Self>> {
        Self::soft_delete_many()?.by_id(id)
    }

    /// Set deleted_at without any filter.
    /// It also checks if the model has configured with deleted_at column or not.
    fn soft_delete_many() -> Res<UpdateMany<Self>> {
        Self::_check_col_deleted_at()?;
        let am = <Self::A as Default>::default()._delete();
        let r = Self::update_many().set(am);
        Ok(r)
    }
}
