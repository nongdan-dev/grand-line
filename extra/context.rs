use sea_orm::*;
// use std::any::{Any, TypeId};
// use std::collections::HashMap;
// use std::sync::{Arc, Mutex};
use std::sync::Arc;

pub struct Context {
    // loaders: Mutex<HashMap<TypeId, Box<dyn Any + Send + Sync>>>,
    pub db: Arc<DatabaseConnection>,
}

impl Context {
    pub fn new(db: DatabaseConnection) -> Self {
        Self {
            // loaders: Mutex::new(HashMap::new()),
            db: Arc::new(db),
        }
    }
}
