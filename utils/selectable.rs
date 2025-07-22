use crate::*;
use sea_orm::*;

/// Helper trait to create sea_orm Select from types like Filter, OrderBy...
pub trait Selectable<E: EntityTrait> {
    /// Helper to create sea_orm Select from types like Filter, OrderBy...
    fn select(&self) -> Select<E>;
}

/// Automatically implement Selectable for Chainable.
impl<T, E> Selectable<E> for T
where
    T: Chainable<E>,
    E: EntityTrait,
{
    fn select(&self) -> Select<E> {
        self.chain(E::find())
    }
}
