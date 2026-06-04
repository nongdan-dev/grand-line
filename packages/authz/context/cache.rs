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

#[derive(Display)]
#[strum(serialize_all = "PascalCase")]
pub enum AuthzCacheOperationTy {
    Query,
    Mutation,
    Subscription,
}
