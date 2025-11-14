use super::prelude::*;

#[async_trait]
pub trait TxContext {
    async fn tx(&self) -> Res<Arc<DatabaseTransaction>>;
}

#[async_trait]
impl TxContext for Context<'_> {
    #[inline(always)]
    async fn tx(&self) -> Res<Arc<DatabaseTransaction>> {
        self.grand_line()?.tx().await
    }
}
