use crate::*;
use async_graphql::dataloader::{DataLoader, Loader};
use sea_orm::*;
use std::collections::HashMap;
use std::sync::{Arc, Weak};
use std::{collections::HashSet, marker::PhantomData};
use tokio::{spawn, sync::Mutex};

struct LoaderF<T>
where
    T: Send + Sync + 'static,
{
    fields: HashSet<String>,
    loader: Arc<DataLoader<T>>,
}

pub struct LoaderCache<T>
where
    T: Send + Sync + 'static,
{
    loaders: Mutex<Vec<LoaderF<T>>>,
}

impl<T> LoaderCache<T>
where
    T: Send + Sync + 'static,
{
    pub fn new() -> Self {
        LoaderCache::<T> {
            loaders: Mutex::new(vec![]),
        }
    }
    pub async fn get(
        &mut self,
        fields: &HashSet<String>,
        create: impl FnOnce(&HashSet<String>) -> T,
    ) -> Arc<DataLoader<T>> {
        let mut loaders = self.loaders.lock().await;

        for a in loaders.iter() {
            if fields.is_subset(&a.fields) {
                return a.loader.clone();
            }
        }

        let l = create(fields);
        let a = Arc::new(DataLoader::new(l, spawn));
        loaders.push(LoaderF {
            fields: fields.clone(),
            loader: a.clone(),
        });
        a
    }
}

pub struct ByColumnLoader<E, C, V> {
    pub ctx: Weak<GrandLineContext>,
    pub fields: HashSet<String>,
    pub column: C,
    pub key_fn: fn(&V) -> String,
    pub _marker: PhantomData<E>,
}
#[async_trait::async_trait]
impl<E, C, V> Loader<String> for ByColumnLoader<E, C, V>
where
    E: EntityTrait + 'static,
    C: ColumnTrait + 'static,
    V: FromQueryResult + Clone + Send + Sync + 'static,
{
    type Value = V;
    type Error = Arc<DbErr>;

    async fn load(&self, keys: &[String]) -> Result<HashMap<String, V>, Self::Error> {
        let ctx = self
            .ctx
            .upgrade()
            .ok_or_else(|| Arc::new(DbErr::Custom("context gone".into())))?;
        let tx = ctx.tx().await?;

        let rows = E::find()
            .filter(self.column.is_in(keys.to_vec()))
            .select_only()
            // TODO:
            .select_column(self.column)
            .into_model::<V>()
            .all(tx.as_ref())
            .await
            .map_err(Arc::new)?;

        Ok(rows.into_iter().map(|v| ((self.key_fn)(&v), v)).collect())
    }
}
