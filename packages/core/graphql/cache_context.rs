use super::prelude::*;

#[async_trait]
pub trait CacheContext<'a>
where
    Self: GrandLineDataContext<'a>,
{
    async fn cache<T, F, Fu>(&self, init: F) -> Res<Arc<T>>
    where
        T: Send + Sync + 'static,
        F: FnOnce() -> Fu + Send,
        Fu: Future<Output = Res<T>> + Send,
    {
        let mut m = self.grand_line()?.cache.lock().await;

        let cell = m.entry(TypeId::of::<T>()).or_insert_with(|| Arc::new(OnceCell::new()));
        let arc = cell
            .get_or_try_init(async move || {
                let arc = Arc::new(init().await?);
                Ok::<_, GrandLineErr>(arc as ArcAny)
            })
            .await?;

        let v = Arc::clone(arc).downcast::<T>().map_err(|_| MyErr::CacheDowncast)?;
        drop(m);

        Ok(v)
    }

    async fn get_cache<T>(&self) -> Res<Option<Arc<T>>>
    where
        T: Send + Sync + 'static,
    {
        let mut m = self.grand_line()?.cache.lock().await;

        let cell = m.entry(TypeId::of::<T>()).or_insert_with(|| Arc::new(OnceCell::new()));
        let Some(arc) = cell.get() else {
            return Ok(None);
        };

        let v = Arc::clone(arc).downcast::<T>().map_err(|_| MyErr::CacheDowncast)?;
        drop(m);

        Ok(Some(v))
    }
}

#[async_trait]
impl<'a> CacheContext<'a> for Context<'a> {
}
