use crate::prelude::*;
use serde::Serialize;

/// Helper trait to combine filter and filter_extra.
pub trait Filter<T>
where
    Self: Conditionable + Send + Sync + Sized + Serialize,
    T: EntityTrait,
{
    /// Combine filter and filter_extra to use in abstract methods.
    /// Should be generated in the #[model] macro.
    fn conf_and(a: Self, b: Self) -> Self;
    /// Check if there is deleted_at in this filter, without the combination of and/or/not.
    /// Should be generated in the #[model] macro.
    fn conf_has_deleted_at(&self) -> bool;
    /// Get and to use in abstract methods.
    /// Should be generated in the #[model] macro.
    fn get_and(&self) -> Option<Vec<Self>>;
    /// Get or to use in abstract methods.
    /// Should be generated in the #[model] macro.
    fn get_or(&self) -> Option<Vec<Self>>;
    /// Get not to use in abstract methods.
    /// Should be generated in the #[model] macro.
    fn get_not(&self) -> Option<Self>;
    /// Check if there is deleted_at in this filter, with the combination of and/or/not.
    fn has_deleted_at(&self) -> bool {
        if self.conf_has_deleted_at() {
            return true;
        }
        if let Some(and) = self.get_and() {
            if and.iter().any(|f| f.has_deleted_at()) {
                return true;
            }
        }
        if let Some(or) = self.get_or() {
            if or.iter().any(|f| f.has_deleted_at()) {
                return true;
            }
        }
        if let Some(not) = self.get_not() {
            if not.has_deleted_at() {
                return true;
            }
        }
        false
    }
}

/// Automatically implement Chainable for Filter
impl<T, F> Chainable<T> for F
where
    T: EntityTrait,
    F: Filter<T>,
{
    fn chain(&self, q: Select<T>) -> Select<T> {
        q.filter(self.cond())
    }
}

/// Automatically implement for Option<Filter>.
pub trait FilterImpl<T>
where
    T: EntityTrait,
{
    /// Helper to combine filter and filter_extra.
    fn combine(self, filter_extra: Self) -> Self;
    /// Check if there is deleted_at in this filter.
    fn has_deleted_at(&self) -> bool;
}

/// Automatically implement for Option<Filter>.
impl<T, F> FilterImpl<T> for Option<F>
where
    T: EntityTrait,
    F: Filter<T>,
{
    fn combine(self, filter_extra: Self) -> Self {
        match (self, filter_extra) {
            (Some(a), Some(b)) => Some(F::conf_and(a, b)),
            (Some(a), None) => Some(a),
            (None, Some(b)) => Some(b),
            (None, None) => None,
        }
    }
    fn has_deleted_at(&self) -> bool {
        self.as_ref()
            .map(|f| f.has_deleted_at())
            .unwrap_or_default()
    }
}
