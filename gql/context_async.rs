use crate::prelude::*;

#[async_trait]
pub trait ContextXAsync
where
    Self: GrandLineContextImpl,
{
    #[inline(always)]
    async fn tx(&self) -> Res<Arc<DatabaseTransaction>> {
        self.grand_line_context()?.tx().await
    }
}

#[async_trait]
impl ContextXAsync for Context<'_> {}
