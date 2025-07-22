use crate::*;

/// Pagination async_graphql input struct to use in search query
#[input]
pub struct Pagination {
    pub limit: Option<u64>,
    pub offset: Option<u64>,
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
