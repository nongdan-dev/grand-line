use crate::*;

/// Pagination async_graphql input struct to use in search query.
#[input]
pub struct Pagination {
    pub limit: Option<u64>,
    pub offset: Option<u64>,
}

/// Helper trait to get offset and limit from pagination, with default and max limit.
pub trait PaginationWith {
    /// Helper to get offset and limit from pagination, with default and max limit.
    fn with(self, limit_default: u64, limit_max: u64) -> (u64, u64);
}

/// Automatically implement PaginationWith for Pagination.
impl PaginationWith for Pagination {
    fn with(self, limit_default: u64, limit_max: u64) -> (u64, u64) {
        (
            if let Some(o) = self.offset { o } else { 0 },
            if let Some(l) = self.limit {
                if l > limit_max { limit_max } else { l }
            } else {
                limit_default
            },
        )
    }
}

/// Automatically implement PaginationWith for Option<Pagination>.
impl PaginationWith for Option<Pagination> {
    fn with(self, limit_default: u64, limit_max: u64) -> (u64, u64) {
        match self {
            Some(p) => p.with(limit_default, limit_max),
            None => (0, limit_default),
        }
    }
}
