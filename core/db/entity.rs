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
    fn model_name() -> &'static str;

    /// Get column id.
    /// Should be generated in the model macro.
    fn col_id() -> Self::C;
    /// Get column created_at.
    /// Should be generated in the model macro.
    fn col_created_at() -> Option<Self::C>;
    /// Get column updated_at.
    /// Should be generated in the model macro.
    fn col_updated_at() -> Option<Self::C>;
    /// Get column deleted_at.
    /// Should be generated in the model macro.
    fn col_deleted_at() -> Option<Self::C>;

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
        if f.len() != 1 {
            Err(MyErr::GqlLookAhead)?;
        }

        let gql_cols = Self::gql_cols();
        let gql_exprs = Self::gql_exprs();
        let gql_select = Self::gql_select();

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

    /// Quickly build condition id eq.
    fn cond_id(id: &str) -> Condition {
        Condition::all().add(Self::col_id().eq(id))
    }

    /// ensure deleted_at column is present.
    fn ensure_col_deleted_at() -> Res<Self::C> {
        let col = Self::col_deleted_at().ok_or_else(|| MyErr::DbCol404 {
            col: Self::model_name().to_owned() + ".deleted_at",
        })?;
        Ok(col)
    }
    /// Quickly build condition include deleted.
    fn cond_deleted_at(include_deleted: Option<bool>) -> Option<Condition> {
        match include_deleted {
            Some(true) => None,
            _ => Self::col_deleted_at().map(|c| Condition::all().add(c.is_null())),
        }
    }

    /// Set deleted_at with filter by id.
    /// It also checks if the model has configured with deleted_at column or not.
    fn soft_delete_by_id(id: &str) -> Res<UpdateMany<Self>> {
        let r = Self::soft_delete_many()?.by_id(id);
        Ok(r)
    }

    /// Set deleted_at without any filter.
    /// It also checks if the model has configured with deleted_at column or not.
    fn soft_delete_many() -> Res<UpdateMany<Self>> {
        Self::ensure_col_deleted_at()?;
        let am = Self::A::defaults_on_delete();
        let r = Self::update_many().set(am);
        Ok(r)
    }
}
