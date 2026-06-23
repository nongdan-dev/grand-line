use crate::prelude::*;
use strum_macros::Display;

#[derive(FromQueryResult)]
pub struct OrgMinimal {
    pub id: String,
}

pub struct AuthzCacheItem {
    pub role: RoleSql,
    pub org: Option<Arc<OrgMinimal>>,
}

/// Will be supplied in macro resolver fn.
#[derive(Display)]
#[strum(serialize_all = "PascalCase")]
pub enum AuthzCacheOperationTy {
    Query,
    Mutation,
    Subscription,
}

/// Type-keyed cache: stores the root operation's field-keyed cache key so that
/// nested resolvers (e.g., relations) can look up the same HashMap entry.
pub struct AuthzCachedKey(pub String);
