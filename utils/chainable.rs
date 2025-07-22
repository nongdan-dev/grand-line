use crate::*;
use sea_orm::*;

/// Helper trait to abstract chain sea_orm query of different types like filter, order_by...
pub trait Chainable<T>
where
    T: EntityTrait,
{
    fn chain(&self, q: Select<T>) -> Select<T>;
}

/// Automatically implement Chainable for Option<Chainable>
impl<T, F> Chainable<T> for Option<F>
where
    T: EntityTrait,
    F: Chainable<T>,
{
    fn chain(&self, q: Select<T>) -> Select<T> {
        match self {
            Some(c) => c.chain(q),
            None => q,
        }
    }
}

/// Automatically implement Chainable for Vec<Chainable>
impl<T, F> Chainable<T> for Vec<F>
where
    T: EntityTrait,
    F: Chainable<T>,
{
    fn chain(&self, q: Select<T>) -> Select<T> {
        let mut q = q;
        for c in self {
            q = c.chain(q)
        }
        q
    }
}
