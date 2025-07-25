use crate::*;
use sea_orm::prelude::*;
use sea_orm::*;

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
    /// This can be used together with the macro grand_line::active_create.
    /// Should be generated in the #[model] macro.
    fn config_active_create(am: A) -> A;
    /// sea_orm ActiveModel hooks will not be called with Entity:: or bulk methods.
    /// We need to have this method instead to get default values on update.
    /// This can be used together with the macro grand_line::active_update.
    /// Should be generated in the #[model] macro.
    fn config_active_update(am: A) -> A;
    /// Get primary id column to use in abstract methods.
    /// Should be generated in the #[model] macro.
    fn config_col_id() -> Self::Column;
    /// Get deleted at column to use in abstract methods.
    /// Should be generated in the #[model] macro.
    fn config_col_deleted_at() -> Option<Self::Column>;
    /// Get sql column from gql field to look ahead
    /// to select only columns from requested fields in the graphql context.
    /// Should be generated in the #[model] macro.
    fn config_gql_col(field: &str) -> Option<Self::Column>;
    /// Get default and max limit configuration.
    /// Should be generated in the #[model] macro.
    fn config_limit() -> (u64, u64);
}
