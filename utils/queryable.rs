use crate::*;
use sea_orm::*;

/// Helper trait to abstract sea_orm query of different types like filter, order_by...
pub trait Queryable<E: EntityTrait> {
    fn query(&self) -> Select<E>;
}

impl<T, E> Queryable<E> for T
where
    T: Chainable<E>,
    E: EntityTrait,
{
    fn query(&self) -> Select<E> {
        self.chain(E::find())
    }
}
