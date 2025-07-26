use crate::prelude::*;
use async_graphql::{Context, extensions::ExtensionContext};

/// GrandLineContext should be constructed on each request.
/// We will get it in the resolvers to manage per-request db transaction, graphql loaders, cache...
/// We should only use it in the GrandLineExtension to inject this context automatically on each request
pub struct GrandLineContext {
    pub(crate) db: Arc<DatabaseConnection>,
    pub(crate) tx: Mutex<Option<Arc<DatabaseTransaction>>>,
}

impl GrandLineContext {
    pub(crate) fn new(ctx: &ExtensionContext<'_>) -> Arc<Self> {
        Arc::new(Self {
            db: ctx.data_unchecked::<Arc<DatabaseConnection>>().clone(),
            tx: Mutex::new(None),
        })
    }

    pub(crate) async fn cleanup(&self, error: bool) -> Res<()> {
        if error {
            self.rollback()
        } else {
            self.commit()
        }
        .await
    }
}

pub trait ContextX {
    fn grand_line_context(&self) -> Arc<GrandLineContext>;
}

impl ContextX for Context<'_> {
    #[inline(always)]
    fn grand_line_context(&self) -> Arc<GrandLineContext> {
        self.data_unchecked::<Arc<GrandLineContext>>().clone()
    }
}

impl ContextX for ExtensionContext<'_> {
    #[inline(always)]
    fn grand_line_context(&self) -> Arc<GrandLineContext> {
        self.data_unchecked::<Arc<GrandLineContext>>().clone()
    }
}
