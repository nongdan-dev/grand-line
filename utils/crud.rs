use crate::Pagination;

/// Helper to combine filter and extra_filter
pub fn filter_combine<T>(a: Option<T>, b: Option<T>, v: &dyn Fn(T, T) -> T) -> Option<T> {
    match (a, b) {
        (Some(a), Some(b)) => Some(v(a, b)),
        (Some(a), None) => Some(a),
        (None, Some(b)) => Some(b),
        (None, None) => None,
    }
}

/// Helper to combine order_by and default_order_by with an initial value if all are empty
pub fn order_by_combine<T>(a: Option<Vec<T>>, b: Option<Vec<T>>, v: T) -> Vec<T> {
    match a {
        Some(a) => match a.len() {
            0 => order_by_combine_opt(b, v),
            _ => a,
        },
        None => order_by_combine_opt(b, v),
    }
}
/// Helper to combine order_by and default_order_by with an initial value if all are empty
fn order_by_combine_opt<T>(a: Option<Vec<T>>, v: T) -> Vec<T> {
    match a {
        Some(a) => order_by_combine_vec(a, v),
        None => vec![v],
    }
}
/// Helper to combine order_by and default_order_by with an initial value if all are empty
fn order_by_combine_vec<T>(a: Vec<T>, v: T) -> Vec<T> {
    match a.len() {
        0 => vec![v],
        _ => a,
    }
}

/// Helper to get pagination with default and max limit
pub fn pagination(p: Option<Pagination>, default_limit: u64, max_limit: u64) -> (u64, u64) {
    match p {
        Some(p) => pagination_some(p, default_limit, max_limit),
        None => (0, default_limit),
    }
}
/// Helper to get pagination with default and max limit
fn pagination_some(p: Pagination, default_limit: u64, max_limit: u64) -> (u64, u64) {
    (
        if let Some(o) = p.offset { o } else { 0 },
        if let Some(l) = p.limit {
            if l > max_limit {
                max_limit
            } else {
                l
            }
        } else {
            default_limit
        },
    )
}
