use sea_orm::*;
// use std::any::{Any, TypeId};
// use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct GrandLineContext {
    pub db: Arc<DatabaseConnection>,
    pub tx: Mutex<Option<Arc<DatabaseTransaction>>>,
    // loaders: Mutex<HashMap<TypeId, Box<dyn Any + Send + Sync>>>,
}

impl GrandLineContext {
    pub fn new(db: DatabaseConnection) -> Self {
        Self {
            db: Arc::new(db),
            tx: Mutex::new(None),
            // loaders: Mutex::new(HashMap::new()),
        }
    }

    pub async fn tx(&self) -> Result<Arc<DatabaseTransaction>, Box<dyn Error + Send + Sync>> {
        let mut r = self.tx.lock().await;
        match &*r {
            Some(a) => Ok(a.clone()),
            None => {
                let a = Arc::new(self.db.begin().await?);
                *r = Some(a.clone());
                Ok(a)
            }
        }
    }

    pub async fn tx_peek(&self) -> Result<Option<()>, Box<dyn Error + Send + Sync>> {
        let r = self.tx.lock().await;
        Ok(r.as_ref().map(|_| ()))
    }

    pub async fn commit(&self) -> Result<(), Box<dyn Error + Send + Sync>> {
        let mut r = self.tx.lock().await;
        if let Some(a) = r.take() {
            match Arc::try_unwrap(a) {
                Ok(tx) => {
                    tx.commit().await?;
                    Ok(())
                }
                Err(_) => Err("Cannot commit: transaction is still in use elsewhere".into()),
            }
        } else {
            Err("No active transaction to commit".into())
        }
    }

    pub async fn rollback(&self) -> Result<(), Box<dyn Error + Send + Sync>> {
        let mut r = self.tx.lock().await;
        if let Some(a) = r.take() {
            match Arc::try_unwrap(a) {
                Ok(tx) => {
                    tx.rollback().await?;
                    Ok(())
                }
                Err(_) => Err("Cannot rollback: transaction is still in use elsewhere".into()),
            }
        } else {
            Err("No active transaction to rollback".into())
        }
    }
}
