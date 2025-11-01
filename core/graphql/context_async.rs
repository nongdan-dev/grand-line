use super::prelude::*;

#[async_trait]
pub trait GrandLineAsyncContext {
    async fn cache<T, F, Fu>(&self, init: F) -> Res<Arc<T>>
    where
        T: Send + Sync + 'static,
        F: FnOnce() -> Fu + Send,
        Fu: Future<Output = Res<T>> + Send;
}

#[async_trait]
impl GrandLineAsyncContext for Context<'_> {
    async fn cache<T, F, Fu>(&self, init: F) -> Res<Arc<T>>
    where
        T: Send + Sync + 'static,
        F: FnOnce() -> Fu + Send,
        Fu: Future<Output = Res<T>> + Send,
    {
        let cell = self
            ._grand_line_context()?
            .cache_others
            .lock()
            .await
            .entry(TypeId::of::<T>())
            .or_insert_with(|| Arc::new(OnceCell::new()))
            .clone();
        let arc = cell
            .get_or_try_init(async move || {
                let arc = Arc::new(init().await?);
                Ok::<_, GrandLineErr>(arc as ArcAny)
            })
            .await?
            .clone()
            .downcast::<T>()
            .map_err(|_| MyErr::CacheDowncast)?;
        Ok(arc)
    }
}
