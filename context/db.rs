use crate::prelude::*;
use async_graphql::Context;

#[async_trait]
pub(crate) trait GrandLineContextDb {
    /// Get or create a sea_orm transaction.
    /// The GrandLineExtension will automatically commit this transaction
    /// if the request executed successfully or rollback if there is an error.
    async fn tx(&self) -> Res<Arc<DatabaseTransaction>>;
    /// Try to rollback if there is an existing transaction in mutex.
    /// We should only use it in the GrandLineExtension to automatically commit the transaction
    /// if the request executed successfully or rollback if there is an error.
    async fn commit(&self) -> Res<()>;
    /// Try to commit if there is an existing transaction in mutex.
    /// We should only use it in the GrandLineExtension to automatically commit the transaction
    /// if the request executed successfully or rollback if there is an error.
    async fn rollback(&self) -> Res<()>;
}

#[async_trait]
impl GrandLineContextDb for GrandLineContext {
    async fn tx(&self) -> Res<Arc<DatabaseTransaction>> {
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

    async fn commit(&self) -> Res<()> {
        match self.tx.lock().await.take() {
            Some(tx) => match Arc::try_unwrap(tx) {
                Ok(tx) => {
                    tx.commit().await?;
                    Ok(())
                }
                Err(_) => err_server!(TxCommit),
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
                Err(_) => err_server!(TxRollback),
            },
            None => Ok(()),
        }
    }
}

#[async_trait]
pub trait ContextXDb: ContextX {
    #[inline(always)]
    async fn tx(&self) -> Res<Arc<DatabaseTransaction>> {
        self.grand_line_context().tx().await
    }
}

#[async_trait]
impl ContextXDb for Context<'_> {}
