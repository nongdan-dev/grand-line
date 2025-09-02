use crate::prelude::*;

/// Pagination async_graphql input struct to use in search query.
#[gql_input]
pub struct Pagination {
    pub offset: Option<u64>,
    pub limit: Option<u64>,
}

/// Pagination after unwrap option into inner.
pub struct PaginationInner {
    pub offset: u64,
    pub limit: u64,
}

/// Pagination limit config.
pub struct ConfigLimit {
    pub default: u64,
    pub max: u64,
}

/// Helper trait to get offset and limit from pagination, with default and max limit.
pub trait ToPaginationInner {
    /// Helper to get offset and limit from pagination, with default and max limit.
    fn inner(self, c: ConfigLimit) -> PaginationInner;
}

/// Automatically implement ToPaginationInner for Pagination.
impl ToPaginationInner for Pagination {
    fn inner(self, c: ConfigLimit) -> PaginationInner {
        PaginationInner {
            offset: self.offset.unwrap_or_default(),
            limit: self
                .limit
                .map(|l| if l > c.max { c.max } else { l })
                .unwrap_or(c.default),
        }
    }
}

/// Automatically implement ToPaginationInner for Option<Pagination>.
impl ToPaginationInner for Option<Pagination> {
    fn inner(self, c: ConfigLimit) -> PaginationInner {
        match self {
            Some(p) => p.inner(c),
            None => PaginationInner {
                offset: 0,
                limit: c.default,
            },
        }
    }
}
