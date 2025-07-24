use crate::*;
use async_graphql::{Context, extensions::ExtensionContext};
use sea_orm::*;

pub struct GrandLineContext {
    pub(crate) db: Arc<DatabaseConnection>,
    pub(crate) tx: Mutex<Option<Arc<DatabaseTransaction>>>,
}

/// GrandLineContext should be constructed on each request.
/// We will get it in the resolvers to manage per-request db transaction, graphql loaders, cache...
/// We should only use it in the GrandLineExtension to inject this context automatically on each request
impl GrandLineContext {
    pub fn new(ctx: &ExtensionContext<'_>) -> Arc<Self> {
        Arc::new(Self {
            db: ctx.data_unchecked::<Arc<DatabaseConnection>>().clone(),
            tx: Mutex::new(None),
        })
    }

    pub fn from(ctx: &Context<'_>) -> Arc<Self> {
        ctx.data_unchecked::<Arc<Self>>().clone()
    }
    pub fn from_extension(ctx: &ExtensionContext<'_>) -> Arc<Self> {
        ctx.data_unchecked::<Arc<Self>>().clone()
    }

    pub async fn cleanup(&self, no_error: bool) -> Res<()> {
        if no_error {
            self.commit().await
        } else {
            self.rollback().await
        }
    }
}
