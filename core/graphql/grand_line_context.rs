use super::prelude::*;

/// GrandLineContext should be constructed on each request.
/// We will get it in the resolvers to manage per-request db transaction, graphql loaders, cache...
/// We should only use it in the GrandLineExtension to inject this context automatically on each request
pub struct GrandLineContext {
    pub(crate) db: Arc<DatabaseConnection>,
    pub(crate) tx: Mutex<Option<Arc<DatabaseTransaction>>>,
    pub(crate) loaders: Mutex<HashMap<String, ArcAny>>,
    pub(crate) cache_others: Mutex<HashMap<TypeId, Arc<OnceCell<ArcAny>>>>,
}
impl GrandLineContext {
    pub(crate) fn new(db: Arc<DatabaseConnection>) -> Self {
        Self {
            db,
            tx: Mutex::new(None),
            loaders: Mutex::new(HashMap::new()),
            cache_others: Mutex::new(HashMap::new()),
        }
    }
    pub(crate) async fn tx(&self) -> Res<Arc<DatabaseTransaction>> {
        let mut guard = self.tx.lock().await;
        match &*guard {
            Some(a) => Ok(a.clone()),
            None => {
                let tx = Arc::new(self.db.begin().await?);
                *guard = Some(tx.clone());
                Ok(tx)
            }
        }
    }

    pub(crate) async fn cleanup(&self, error: bool) -> Res<()> {
        self.loaders.lock().await.clear();
        if error {
            self.rollback().await
        } else {
            self.commit().await
        }
    }

    async fn commit(&self) -> Res<()> {
        if let Some(tx) = self.tx.lock().await.take() {
            match Arc::try_unwrap(tx) {
                Ok(tx) => {
                    tx.commit().await?;
                }
                Err(_) => {
                    err!(TxCommit)?;
                }
            }
        }
        Ok(())
    }

    async fn rollback(&self) -> Res<()> {
        if let Some(tx) = self.tx.lock().await.take() {
            match Arc::try_unwrap(tx) {
                Ok(tx) => {
                    tx.rollback().await?;
                }
                Err(_) => {
                    err!(TxRollback)?;
                }
            }
        }
        Ok(())
    }
}
