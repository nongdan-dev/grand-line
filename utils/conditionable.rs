use crate::*;
use sea_orm::*;

/// Helper trait to create sea_orm condition from different types like Filter...
pub trait Conditionable {
    fn condition(&self) -> Condition;
}
