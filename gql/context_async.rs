use crate::prelude::*;
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

    async fn load_data<E>(&self, col: E::C, id: String) -> Res<Option<E::G>>
    where
        E: EntityX;
}

#[async_trait]
impl ContextXAsync for Context<'_> {
    async fn load_data<E>(&self, col: E::C, id: String) -> Res<Option<E::G>>
    where
        E: EntityX,
    {
        let look_ahead = E::gql_look_ahead(self)?;
        let key = col.to_loader_key(&look_ahead);
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
                },
                tokio::spawn,
            ));
            guard.insert(key, a.clone());
            a
        };
        a.as_ref().load_one(id).await
    }
}
