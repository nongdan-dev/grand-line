use super::prelude::*;

/// GrandLineContext should be constructed on each request.
/// We will get it in the resolvers to manage per-request db transaction, graphql loaders, cache...
/// We should only use it in the GrandLineExtension to inject this context automatically on each request
pub struct GrandLineContext {
    pub(crate) db: Arc<DatabaseConnection>,
    pub(crate) tx: Mutex<Option<Arc<DatabaseTransaction>>>,
    pub(crate) loaders: Mutex<HashMap<String, Arc<dyn Any + Send + Sync>>>,
}

impl GrandLineContext {
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
        match self.tx.lock().await.take() {
            Some(tx) => match Arc::try_unwrap(tx) {
                Ok(tx) => {
                    tx.commit().await?;
                }
                Err(_) => {
                    err!(TxCommit)?;
                }
            },
            None => {}
        }
        Ok(())
    }

    async fn rollback(&self) -> Res<()> {
        match self.tx.lock().await.take() {
            Some(tx) => match Arc::try_unwrap(tx) {
                Ok(tx) => {
                    tx.rollback().await?;
                }
                Err(_) => {
                    err!(TxRollback)?;
                }
            },
            None => {}
        }
        Ok(())
    }
}

pub trait GrandLineContextImpl {
    fn grand_line_context(&self) -> Res<Arc<GrandLineContext>>;
}

impl GrandLineContextImpl for Context<'_> {
    #[inline(always)]
    fn grand_line_context(&self) -> Res<Arc<GrandLineContext>> {
        map_err(self.data::<Arc<GrandLineContext>>())
    }
}

impl GrandLineContextImpl for ExtensionContext<'_> {
    #[inline(always)]
    fn grand_line_context(&self) -> Res<Arc<GrandLineContext>> {
        map_err(self.data::<Arc<GrandLineContext>>())
    }
}

fn map_err(r: Result<&Arc<GrandLineContext>, GraphQLErr>) -> Res<Arc<GrandLineContext>> {
    let a = r.cloned().map_err(|e| MyErr::Ctx404 { inner: e.message })?;
    Ok(a)
}
