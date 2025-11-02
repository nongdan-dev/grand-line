use super::prelude::*;

#[async_trait]
pub trait GrandLineTxContextAsync {
    async fn tx(&self) -> Res<Arc<DatabaseTransaction>>;
}

#[async_trait]
impl GrandLineTxContextAsync for Context<'_> {
    #[inline(always)]
    async fn tx(&self) -> Res<Arc<DatabaseTransaction>> {
        self.grand_line_context()?.tx().await
    }
}
