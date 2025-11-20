use crate::prelude::*;
use strum_macros::Display;

#[derive(FromQueryResult)]
pub struct OrgMinimal {
    pub id: String,
}
impl OrgMinimal {
    pub fn select(q: Select<Org>) -> Selector<SelectModel<Self>> {
        q.select_only()
            .column(OrgColumn::Id)
            .into_model::<OrgMinimal>()
    }
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
