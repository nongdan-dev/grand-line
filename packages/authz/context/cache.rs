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
/// Per-request cache for authz results.
pub type AuthzCache = Mutex<HashMap<String, Arc<Option<AuthzCacheItem>>>>;

/// Per-request cache for authz_row results, keyed by (filter TypeId, field path).
/// Avoids calling the handler repeatedly for the same field in the same request
/// (e.g. N parents each resolving the same has_one relation with row auth).
/// This seems to be a generic type, so we need to create a struct wrapper to avoid conflict.
pub struct AuthzRowCache(pub Mutex<HashMap<(TypeId, String), ArcAny>>);
