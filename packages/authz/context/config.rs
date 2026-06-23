use crate::prelude::*;

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
pub trait AuthzHandlers: Send + Sync {
    async fn on_row_script(&self, ctx: &Context<'_>) -> Res<Option<JsonValue>> {
        Ok(None)
    }
}

struct DefaultHandlers;
#[async_trait]
impl AuthzHandlers for DefaultHandlers {}

/// Org lookup callbacks, non-generic: method signatures use only primitives
/// so the trait needs no type parameter.
/// Use `AuthzOrgImpl::default::<YourOrg>()` to build with the default DB lookup,
/// or `AuthzOrgImpl::new(MyHandlers)` to provide a custom implementation.
/// Add this to your schema with `.data(AuthzOrgImpl::default::<YourOrg>())`.
pub struct AuthzOrgImpl {
    pub handlers: Arc<dyn AuthzOrgImplHandlers>,
}

impl AuthzOrgImpl {
    pub fn new(handlers: impl AuthzOrgImplHandlers) -> Self {
        Self {
            handlers: Arc::new(handlers),
        }
    }

    pub fn default<O: AuthzOrg>() -> Self {
        Self {
            handlers: Arc::new(DefaultOrgImplHandlers::<O>(PhantomData)),
        }
    }
}

#[async_trait]
pub trait AuthzOrgImplHandlers: Send + Sync {
    async fn find_by_id(&self, id: &str, tx: &DatabaseTransaction) -> Res<Option<OrgMinimal>>;
}

struct DefaultOrgImplHandlers<O>(PhantomData<O>);
#[async_trait]
impl<O: AuthzOrg> AuthzOrgImplHandlers for DefaultOrgImplHandlers<O> {
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
