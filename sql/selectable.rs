use crate::prelude::*;

/// Helper trait to create sea_orm Select from types like Filter, OrderBy...
pub trait Selectable<E: EntityX> {
    /// Helper to create sea_orm Select from types like Filter, OrderBy...
    fn select(&self) -> Select<E>;
}

/// Automatically implement Selectable for Chainable.
impl<E, C> Selectable<E> for C
where
    E: EntityX,
    C: Chainable<E>,
{
    fn select(&self) -> Select<E> {
        self.chain(E::find())
    }
}
