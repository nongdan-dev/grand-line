/// Helper trait to combine filter and extra_filter
pub trait Filter
where
    Self: Sized,
{
    fn combine(a: Self, b: Self) -> Self;
}

/// Automatically implement combine for Option<T>
pub trait FilterImpl {
    fn combine(self, extra_filter: Self) -> Self;
}

/// Automatically implement combine for Option<T>
impl<T> FilterImpl for Option<T>
where
    T: Filter,
{
    fn combine(self, extra_filter: Self) -> Self {
        match (self, extra_filter) {
            (Some(a), Some(b)) => Some(T::combine(a, b)),
            (Some(a), None) => Some(a),
            (None, Some(b)) => Some(b),
            (None, None) => None,
        }
    }
}
