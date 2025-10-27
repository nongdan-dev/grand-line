use super::prelude::*;
use serde::Serialize;

/// Helper trait to combine filter and filter_extra.
pub trait Filter<E>
where
    E: EntityX,
    Self: IntoCondition + Default + Serialize + Send + Sync + Sized,
{
    /// Combine filter and filter_extra to use in abstract methods.
    /// Should be generated in the model macro.
    fn _combine_and(a: Self, b: Self) -> Self;
    /// Check if there is deleted_at in this filter, without the combination of and/or/not.
    /// Should be generated in the model macro.
    fn _has_deleted_at(&self) -> bool;
    /// Get and to use in abstract methods.
    /// Should be generated in the model macro.
    fn _get_and(&self) -> Option<Vec<Self>>;
    /// Get or to use in abstract methods.
    /// Should be generated in the model macro.
    fn _get_or(&self) -> Option<Vec<Self>>;
    /// Get not to use in abstract methods.
    /// Should be generated in the model macro.
    fn _get_not(&self) -> Option<Self>;

    /// Check if there is deleted_at in this filter, with the combination of and/or/not.
    fn has_deleted_at(&self) -> bool {
        if self._has_deleted_at() {
            return true;
        }
        if let Some(and) = self._get_and()
            && and.iter().any(|f| f.has_deleted_at())
        {
            return true;
        }
        if let Some(or) = self._get_or()
            && or.iter().any(|f| f.has_deleted_at())
        {
            return true;
        }
        if let Some(not) = self._get_not()
            && not.has_deleted_at()
        {
            return true;
        }
        false
    }
}

/// Automatically implement ChainSelect for Filter
impl<E, F> ChainSelect<E> for F
where
    E: EntityX,
    F: Filter<E>,
{
    fn chain_select(self, q: Select<E>) -> Select<E> {
        q.filter(self.into_condition())
    }
}

/// Automatically implement for Option<Filter>.
pub trait FilterImpl<E>
where
    E: EntityX,
{
    /// Helper to combine filter and filter_extra.
    fn combine(self, filter_extra: Self) -> Self;
    /// Check if there is deleted_at in this filter.
    fn has_deleted_at(&self) -> bool;
}

/// Automatically implement for Option<Filter>.
impl<E, F> FilterImpl<E> for Option<F>
where
    E: EntityX,
    F: Filter<E>,
{
    fn combine(self, filter_extra: Self) -> Self {
        match (self, filter_extra) {
            (Some(a), Some(b)) => Some(F::_combine_and(a, b)),
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
