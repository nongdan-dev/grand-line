use crate::prelude::*;

#[derive(Clone)]
pub struct AuthzConfig {
    pub org_id_header_key: &'static str,
    pub handlers: Arc<dyn AuthzHandlers>,
}

impl Default for AuthzConfig {
    fn default() -> Self {
        Self {
            org_id_header_key: "x-org-id",
            handlers: Arc::new(DefaultHandlers),
        }
    }
}

#[allow(unused_variables)]
#[async_trait]
pub trait AuthzHandlers
where
    Self: Send + Sync,
{
}

struct DefaultHandlers;
#[async_trait]
impl AuthzHandlers for DefaultHandlers {}
