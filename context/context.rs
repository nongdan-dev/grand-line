use crate::*;
use async_graphql::extensions::ExtensionContext;
use async_graphql::Context;
use sea_orm::*;
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::LoaderCache;

pub struct GrandLineContext {
    db: Arc<DatabaseConnection>,
    tx: Mutex<Option<Arc<DatabaseTransaction>>>,
    loaders: Mutex<HashMap<TypeId, Box<dyn Any + Send + Sync>>>,
}

/// GrandLineContext should be constructed on each request.
/// We will get it in the resolvers to manage per-request db transaction, graphql loaders, cache...
/// We should only use it in the GrandLineExtension to inject this context automatically on each request
impl GrandLineContext {
    pub fn new(ctx: &ExtensionContext<'_>) -> Arc<Self> {
        Arc::new(Self {
            // TODO: move this to a new DbContext
            db: ctx.data_unchecked::<Arc<DatabaseConnection>>().clone(),
            tx: Mutex::new(None),
            loaders: Mutex::new(HashMap::new()),
        })
    }
    // TODO: add clean up

    pub fn from(ctx: &Context<'_>) -> Arc<Self> {
        ctx.data_unchecked::<Arc<Self>>().clone()
    }
    pub fn from_extension(ctx: &ExtensionContext<'_>) -> Arc<Self> {
        ctx.data_unchecked::<Arc<Self>>().clone()
    }

    /// Get or create a sea_orm transaction.
    /// The GrandLineExtension will automatically commit this transaction
    /// if the request executed successfully or rollback if there is an error
    pub async fn tx(&self) -> Result<Arc<DatabaseTransaction>, DbErr> {
        let mut guard = self.tx.lock().await;
        match &*guard {
            Some(a) => Ok(a.clone()),
            None => {
                let a = Arc::new(self.db.begin().await?);
                *guard = Some(a.clone());
                Ok(a)
            }
        }
    }

    /// Try to commit if there is an existing transaction in mutex.
    /// We should only use it in the GrandLineExtension to automatically commit the transaction
    /// if the request executed successfully or rollback if there is an error
    pub async fn commit(&self) -> Result<(), Box<dyn Error + Send + Sync>> {
        match self.tx.lock().await.take() {
            Some(a) => match Arc::try_unwrap(a) {
                Ok(tx) => {
                    tx.commit().await?;
                    Ok(())
                }
                Err(_) => Err("Cannot commit: transaction is still in use elsewhere".into()),
            },
            None => Ok(()),
        }
    }

    /// Try to rollback if there is an existing transaction in mutex.
    /// We should only use it in the GrandLineExtension to automatically commit the transaction
    /// if the request executed successfully or rollback if there is an error
    pub async fn rollback(&self) -> Result<(), Box<dyn Error + Send + Sync>> {
        match self.tx.lock().await.take() {
            Some(a) => match Arc::try_unwrap(a) {
                Ok(tx) => {
                    tx.rollback().await?;
                    Ok(())
                }
                Err(_) => Err("Cannot rollback: transaction is still in use elsewhere".into()),
            },
            None => Ok(()),
        }
    }

    pub async fn loader<T>(&self) -> Result<Arc<LoaderCache<T>>, Box<dyn Error + Send + Sync>>
    where
        T: Send + Sync + 'static,
    {
        let ty = TypeId::of::<T>();
        let mut guard = self.loaders.lock().await;
        match guard.get(&ty) {
            Some(a) => match a.downcast_ref::<Arc<LoaderCache<T>>>() {
                Some(a) => Ok(a.clone()),
                None => Err("Cannot downcast ref: Arc<LoaderCache<T>>".into()),
            },
            None => {
                let a = Arc::new(LoaderCache::<T>::new());
                guard.insert(ty, Box::new(a.clone()));
                Ok(a)
            }
        }
    }
}
