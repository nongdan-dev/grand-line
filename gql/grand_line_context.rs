use crate::prelude::*;

/// GrandLineContext should be constructed on each request.
/// We will get it in the resolvers to manage per-request db transaction, graphql loaders, cache...
/// We should only use it in the GrandLineExtension to inject this context automatically on each request
pub struct GrandLineContext {
    pub(crate) db: Arc<DatabaseConnection>,
    pub(crate) tx: Mutex<Option<Arc<DatabaseTransaction>>>,
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
                    Ok(())
                }
                Err(_) => err_server_res!(TxCommit),
            },
            None => Ok(()),
        }
    }

    async fn rollback(&self) -> Res<()> {
        match self.tx.lock().await.take() {
            Some(tx) => match Arc::try_unwrap(tx) {
                Ok(tx) => {
                    tx.rollback().await?;
                    Ok(())
                }
                Err(_) => err_server_res!(TxRollback),
            },
            None => Ok(()),
        }
    }
}

pub trait GrandLineContextImpl {
    fn grand_line_context(&self) -> Res<Arc<GrandLineContext>>;
}

impl GrandLineContextImpl for Context<'_> {
    #[inline(always)]
    fn grand_line_context(&self) -> Res<Arc<GrandLineContext>> {
        try_unwrap(self.data::<Arc<GrandLineContext>>())
    }
}

impl GrandLineContextImpl for ExtensionContext<'_> {
    #[inline(always)]
    fn grand_line_context(&self) -> Res<Arc<GrandLineContext>> {
        try_unwrap(self.data::<Arc<GrandLineContext>>())
    }
}

fn try_unwrap(r: Result<&Arc<GrandLineContext>, Error>) -> Res<Arc<GrandLineContext>> {
    r.cloned().map_err(|e| err_server!(Ctx404(e.message)))
}
