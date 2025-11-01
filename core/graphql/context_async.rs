use super::prelude::*;

#[async_trait]
pub trait GrandLineAsyncContext {
    async fn get_cache<T>(&self) -> Res<Option<Arc<T>>>
    where
        T: 'static + Send + Sync;

    async fn cache<T>(&self, v: T) -> Res<Arc<T>>
    where
        T: 'static + Send + Sync;
}

#[async_trait]
impl GrandLineAsyncContext for Context<'_> {
    async fn get_cache<T>(&self) -> Res<Option<Arc<T>>>
    where
        T: 'static + Send + Sync,
    {
        let any = self
            ._grand_line_context()?
            .cache_others
            .lock()
            .await
            .get(&TypeId::of::<T>())
            .cloned();
        let any = if let Some(any) = any {
            any
        } else {
            return Ok(None);
        };
        let arc = any
            .clone()
            .downcast::<T>()
            .map_err(|_| MyErr::CacheDowncast)?;
        Ok(Some(arc))
    }

    async fn cache<T>(&self, v: T) -> Res<Arc<T>>
    where
        T: 'static + Send + Sync,
    {
        let arc = Arc::new(v);
        self._grand_line_context()?
            .cache_others
            .lock()
            .await
            .insert(TypeId::of::<T>(), arc.clone());
        Ok(arc)
    }
}
