use crate::*;
use sea_orm::*;

/// Helper trait to combine filter and filter_extra.
pub trait Filter<T>
where
    Self: Conditionable + Send + Sync + Sized,
    T: EntityTrait,
{
    /// Combine filter and filter_extra to use in abstract methods.
    /// Should will be generated in the macro.
    fn combine(a: Self, b: Self) -> Self;
}

/// Automatically implement Chainable for Filter
impl<T, F> Chainable<T> for F
where
    T: EntityTrait,
    F: Filter<T>,
{
    fn chain(&self, q: Select<T>) -> Select<T> {
        q.filter(self.condition())
    }
}

/// Automatically implement combine for Option<Filter>.
pub trait FilterImpl<T>
where
    T: EntityTrait,
{
    /// Helper to combine filter and filter_extra.
    fn combine(self, filter_extra: Self) -> Self;
}

/// Automatically implement combine for Option<Filter>.
impl<T, F> FilterImpl<T> for Option<F>
where
    T: EntityTrait,
    F: Filter<T>,
{
    fn combine(self, filter_extra: Self) -> Self {
        match (self, filter_extra) {
            (Some(a), Some(b)) => Some(F::combine(a, b)),
            (Some(a), None) => Some(a),
            (None, Some(b)) => Some(b),
            (None, None) => None,
        }
    }
}
