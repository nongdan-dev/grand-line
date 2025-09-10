use crate::prelude::*;

/// Helper trait to chain sea_orm Select of different types like Filter, OrderBy...
pub trait Chainable<E>
where
    E: EntityX,
{
    /// Helper to chain sea_orm Select of different types like Filter, OrderBy...
    fn chain(&self, q: Select<E>) -> Select<E>;
}

/// Automatically implement Chainable for Option<Chainable>.
impl<E, C> Chainable<E> for Option<C>
where
    E: EntityX,
    C: Chainable<E>,
{
    fn chain(&self, q: Select<E>) -> Select<E> {
        match self {
            Some(c) => c.chain(q),
            None => q,
        }
    }
}

/// Automatically implement Chainable for Vec<Chainable>.
impl<E, C> Chainable<E> for Vec<C>
where
    E: EntityX,
    C: Chainable<E>,
{
    fn chain(&self, mut q: Select<E>) -> Select<E> {
        for c in self {
            q = c.chain(q)
        }
        q
    }
}

/// Automatically implement Chainable for PaginationInner.
impl<E> Chainable<E> for PaginationInner
where
    E: EntityX,
{
    fn chain(&self, q: Select<E>) -> Select<E> {
        q.offset(self.offset).limit(self.limit)
    }
}
