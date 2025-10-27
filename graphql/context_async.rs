use super::prelude::*;
use async_graphql::dataloader::DataLoader;

#[async_trait]
pub trait ContextXAsync
where
    Self: GrandLineContextImpl,
{
    #[inline(always)]
    async fn tx(&self) -> Res<Arc<DatabaseTransaction>> {
        self.grand_line_context()?.tx().await
    }
    async fn data_loader<E>(
        &self,
        key: String,
        col: E::C,
        look_ahead: Vec<LookaheadX<E>>,
        include_deleted: Option<Condition>,
    ) -> Res<Arc<DataLoader<LoaderX<E>>>>
    where
        E: EntityX;
}

#[async_trait]
impl ContextXAsync for Context<'_> {
    async fn data_loader<E>(
        &self,
        key: String,
        col: E::C,
        look_ahead: Vec<LookaheadX<E>>,
        include_deleted: Option<Condition>,
    ) -> Res<Arc<DataLoader<LoaderX<E>>>>
    where
        E: EntityX,
    {
        let gl = self.grand_line_context()?;
        let mut guard = gl.loaders.lock().await;
        let a = if let Some(a) = guard.get(&key) {
            a.clone()
                .downcast::<DataLoader<LoaderX<E>>>()
                .map_err(|_| MyErr::LoaderDowncast)?
        } else {
            let a = Arc::new(DataLoader::new(
                LoaderX {
                    tx: gl.tx().await?,
                    col,
                    look_ahead,
                    include_deleted,
                },
                tokio::spawn,
            ));
            guard.insert(key, a.clone());
            a
        };
        Ok(a)
    }
}
