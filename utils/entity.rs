use crate::prelude::*;

/// Helper trait to abstract extra methods into sea_orm entity.
pub trait EntityX<M, A, F, O, G>
where
    Self: EntityTrait<Model = M>,
    M: FromQueryResult + Send + Sync,
    A: ActiveModelTrait<Entity = Self>,
    F: Filter<Self>,
    O: OrderBy<Self>,
    G: FromQueryResult + Send + Sync,
{
    /// sea_orm ActiveModel hooks will not be called with Entity:: or bulk methods.
    /// We need to have this method instead to get default values on create.
    /// This can be used together with the macro grand_line::am_create.
    /// Should be generated in the #[model] macro.
    fn config_am_create(am: A) -> A;
    /// sea_orm ActiveModel hooks will not be called with Entity:: or bulk methods.
    /// We need to have this method instead to get default values on update.
    /// This can be used together with the macro grand_line::am_update.
    /// Should be generated in the #[model] macro.
    fn config_am_update(am: A) -> A;
    /// Get primary id column to use in abstract methods.
    /// Should be generated in the #[model] macro.
    fn config_col_id() -> Self::Column;
    /// Get deleted at column to use in abstract methods.
    /// Should be generated in the #[model] macro.
    fn config_col_deleted_at() -> Option<Self::Column>;
    /// Get sql column from gql field to look ahead
    /// to select only columns from requested fields in the graphql context.
    /// Should be generated in the #[model] macro.
    fn config_gql_select(
        field: &str,
    ) -> (
        Option<Vec<Self::Column>>,
        Option<(String, sea_query::SimpleExpr)>,
    );
    /// Get default and max limit configuration.
    /// Should be generated in the #[model] macro.
    fn config_limit() -> ConfigLimit;
}
