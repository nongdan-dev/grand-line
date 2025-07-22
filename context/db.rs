use crate::*;
use async_trait::async_trait;
use sea_orm::*;
use std::sync::Arc;

#[async_trait]
pub trait GrandLineContextDbImpl {
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
impl GrandLineContextDbImpl for GrandLineContext {
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
                Err(_) => Err(GrandLineError::TxCommit),
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
                Err(_) => Err(GrandLineError::TxRollback),
            },
            None => Ok(()),
        }
    }
}
