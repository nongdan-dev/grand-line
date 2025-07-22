use crate::*;
use sea_orm::*;

/// Helper trait to abstract sea_orm condition of different types like filter...
pub trait Conditionable {
    fn condition(&self) -> Condition;
}
