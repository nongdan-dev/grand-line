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

/// Generic org config: callbacks that receive the org's own model type.
/// Use `let org_impl = AuthzOrgImpl::<Org>::default()` for no-op handlers,
/// or `let org_impl = AuthzOrgImpl::<Org>::new(MyHandlers)` to provide custom callbacks.
/// Add this to your schema with `.data(org_impl)`.
pub struct AuthzOrgImpl<O>
where
    O: AuthzOrg,
{
    pub handlers: Arc<dyn AuthzOrgImplHandlers<O>>,
}

impl<O> AuthzOrgImpl<O>
where
    O: AuthzOrg,
{
    pub fn new(handlers: impl AuthzOrgImplHandlers<O> + 'static) -> Self {
        Self {
            handlers: Arc::new(handlers),
        }
    }
}

impl<O> Default for AuthzOrgImpl<O>
where
    O: AuthzOrg,
{
    fn default() -> Self {
        Self {
            handlers: Arc::new(DefaultUserImplHandlers(PhantomData)),
        }
    }
}

#[allow(unused_variables)]
#[async_trait]
pub trait AuthzOrgImplHandlers<O>
where
    O: AuthzOrg,
    Self: Send + Sync,
{
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

struct DefaultUserImplHandlers<O>(PhantomData<O>);
#[async_trait]
impl<O> AuthzOrgImplHandlers<O> for DefaultUserImplHandlers<O> where O: AuthzOrg {}
