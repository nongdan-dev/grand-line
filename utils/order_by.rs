/// Helper trait to combine order_by and default_order_by with an initial value if all are empty
pub trait OrderBy
where
    Self: Sized,
{
    fn default() -> Self;
}

/// Automatically implement combine for Option<Vec<T>>
pub trait OrderByImpl<T> {
    fn combine(self, default_order_by: Self) -> Vec<T>;
}

/// Automatically implement combine for Option<Vec<T>>
impl<T> OrderByImpl<T> for Option<Vec<T>>
where
    T: OrderBy,
{
    fn combine(self, default_order_by: Self) -> Vec<T> {
        match self {
            Some(a) => match a.len() {
                0 => opt(default_order_by, T::default()),
                _ => a,
            },
            None => opt(default_order_by, T::default()),
        }
    }
}

fn opt<T>(a: Option<Vec<T>>, v: T) -> Vec<T> {
    match a {
        Some(a) => vec(a, v),
        None => vec![v],
    }
}
fn vec<T>(a: Vec<T>, v: T) -> Vec<T> {
    match a.len() {
        0 => vec![v],
        _ => a,
    }
}
