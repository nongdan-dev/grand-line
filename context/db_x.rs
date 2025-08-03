use crate::prelude::*;
use async_graphql::Context;

#[async_trait]
pub trait ContextXDb: ContextX {
    #[inline(always)]
    async fn tx(&self) -> Res<Arc<DatabaseTransaction>> {
        self.grand_line_context().tx().await
    }
}

#[async_trait]
impl ContextXDb for Context<'_> {}
