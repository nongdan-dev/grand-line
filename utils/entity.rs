use crate::*;
use sea_orm::prelude::*;
use sea_orm::*;

/// Helper trait to abstract extra methods into sea_orm entity.
pub trait EntityX<M, F, O, R>
where
    Self: EntityTrait<Model = M>,
    M: FromQueryResult + Send + Sync,
    F: Filter<Self>,
    O: OrderBy<Self>,
    R: FromQueryResult + Send + Sync,
{
    /// Get primary id column to use in abstract methods.
    /// Should will be generated in the macro.
    fn id() -> Self::Column;
    /// Get sql column from gql field to look ahead
    /// to select only columns from requested fields in the graphql context.
    /// Should will be generated in the macro.
    fn column(field: &str) -> Option<Self::Column>;
    /// Get default and max limit configuration.
    /// Should will be generated in the macro if there are macro attributes:
    /// limit_default = 100, limit_max = 1000
    fn config_limit() -> (u64, u64) {
        (100, 1000)
    }
}
