use super::prelude::*;

#[async_trait]
pub trait CacheContext {
    async fn cache<T, F, Fu>(&self, init: F) -> Res<Arc<T>>
    where
        T: Send + Sync + 'static,
        F: FnOnce() -> Fu + Send,
        Fu: Future<Output = Res<T>> + Send;
    async fn get_cache<T>(&self) -> Res<Option<Arc<T>>>
    where
        T: Send + Sync + 'static;
}

#[async_trait]
impl CacheContext for Context<'_> {
    async fn cache<T, F, Fu>(&self, init: F) -> Res<Arc<T>>
    where
        T: Send + Sync + 'static,
        F: FnOnce() -> Fu + Send,
        Fu: Future<Output = Res<T>> + Send,
    {
        let mut mutex = self.grand_line()?.cache.lock().await;
        let cell = mutex
            .entry(TypeId::of::<T>())
            .or_insert_with(|| Arc::new(OnceCell::new()));
        let arc = cell
            .get_or_try_init(async move || {
                let arc = Arc::new(init().await?);
                Ok::<_, GrandLineErr>(arc as ArcAny)
            })
            .await?;
        let v = arc
            .clone()
            .downcast::<T>()
            .map_err(|_| MyErr::CacheDowncast)?;
        Ok(v)
    }
    async fn get_cache<T>(&self) -> Res<Option<Arc<T>>>
    where
        T: Send + Sync + 'static,
    {
        let mut mutex = self.grand_line()?.cache.lock().await;
        let cell = mutex
            .entry(TypeId::of::<T>())
            .or_insert_with(|| Arc::new(OnceCell::new()));
        let Some(arc) = cell.get() else {
            return Ok(None);
        };
        let v = arc
            .clone()
            .downcast::<T>()
            .map_err(|_| MyErr::CacheDowncast)?;
        Ok(Some(v))
    }
}
