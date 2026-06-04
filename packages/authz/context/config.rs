use crate::prelude::*;
use std::marker::PhantomData;

#[derive(Clone)]
pub struct AuthzConfig {
    pub org_id_header_key: &'static str,
    pub handlers: Arc<dyn AuthzHandlers>,
}

impl Default for AuthzConfig {
    fn default() -> Self {
        Self {
            org_id_header_key: H_ORG_ID,
            handlers: Arc::new(DefaultHandlers),
        }
    }
}

#[allow(unused_variables)]
#[async_trait]
pub trait AuthzHandlers: Send + Sync {}

struct DefaultHandlers;
#[async_trait]
impl AuthzHandlers for DefaultHandlers {}

/// Type-erased org lookup - stored in context so proc-macro resolvers can
/// use it without needing to know the generic `O` type parameter.
/// Register with `.data(authz_org::<YourOrg>())` when building your schema.
#[async_trait]
pub trait AuthzOrgLookup: Send + Sync {
    async fn find_by_id(&self, id: &str, tx: &DatabaseTransaction) -> Res<Option<OrgMinimal>>;
}

struct DefaultOrgLookup<O: AuthzOrg>(PhantomData<O>);

#[async_trait]
impl<O: AuthzOrg> AuthzOrgLookup for DefaultOrgLookup<O> {
    async fn find_by_id(&self, id: &str, tx: &DatabaseTransaction) -> Res<Option<OrgMinimal>> {
        let r = O::find()
            .exclude_deleted()
            .filter_by_id(id)
            .select_only()
            .column(O::col_id())
            .into_model::<OrgMinimal>()
            .one(tx)
            .await?;
        Ok(r)
    }
}

/// Create an org lookup for use in `.data(authz_org::<YourOrg>())`.
pub fn authz_org<O: AuthzOrg>() -> Arc<dyn AuthzOrgLookup> {
    Arc::new(DefaultOrgLookup::<O>(PhantomData))
}
