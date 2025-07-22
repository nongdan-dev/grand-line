use crate::*;
use sea_orm::*;

/// Helper trait to combine order_by and order_by_default with an initial value if all are empty.
pub trait OrderBy<T>
where
    Self: Chainable<T> + Send + Sync + Sized,
    T: EntityTrait,
{
    /// Get order_by_default to use in abstract methods.
    /// Should will be generated in the macro.
    fn default() -> Self;
}

/// Automatically implement combine for Option<Vec<OrderBy>>.
pub trait OrderByImpl<T, O>
where
    T: EntityTrait,
    O: OrderBy<T>,
{
    /// Helper to combine order_by and order_by_default with an initial value if all are empty.
    fn combine(self, order_by_default: Self) -> Vec<O>;
}

/// Automatically implement combine for Option<Vec<OrderBy>>.
impl<T, O> OrderByImpl<T, O> for Option<Vec<O>>
where
    T: EntityTrait,
    O: OrderBy<T>,
{
    fn combine(self, order_by_default: Self) -> Vec<O> {
        match self {
            Some(o) => match o.len() {
                0 => opt(order_by_default, O::default()),
                _ => o,
            },
            None => opt(order_by_default, O::default()),
        }
    }
}

fn opt<O>(o: Option<Vec<O>>, order_by_default: O) -> Vec<O> {
    match o {
        Some(a) => vec(a, order_by_default),
        None => vec![order_by_default],
    }
}
fn vec<O>(o: Vec<O>, order_by_default: O) -> Vec<O> {
    match o.len() {
        0 => vec![order_by_default],
        _ => o,
    }
}
