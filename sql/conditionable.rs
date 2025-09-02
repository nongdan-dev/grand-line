use crate::prelude::*;

/// Helper trait to create sea_orm condition from different types like Filter...
pub trait Conditionable {
    fn cond(&self) -> Condition;
}
