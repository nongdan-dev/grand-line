use sea_orm::entity::prelude::*;
use sea_orm::*;

pub trait Conditionable {
    fn condition(&self) -> Condition;
}

pub trait Chainable<T>
where
    T: EntityTrait,
{
    fn chain(&self, q: Select<T>) -> Select<T>;
}
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
