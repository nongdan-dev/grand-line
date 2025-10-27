use super::prelude::*;

/// Helper trait to create sea_orm condition from different types like Filter...
pub trait IntoCondition {
    fn into_condition(self) -> Condition;
}
