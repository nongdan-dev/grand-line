use super::prelude::*;
use async_graphql::dataloader::Loader;

pub struct LoaderX<E>
where
    E: EntityX,
{
    pub tx: Arc<DatabaseTransaction>,
    pub col: E::C,
    pub look_ahead: Vec<LookaheadX<E>>,
    pub include_deleted: Option<Condition>,
}

#[async_trait]
impl<E> Loader<String> for LoaderX<E>
where
    E: EntityX,
{
    type Value = E::G;
    type Error = GrandLineErr;

    async fn load(&self, keys: &[String]) -> Res<HashMap<String, E::G>> {
        let tx = self.tx.as_ref();
        let mut r = E::find();
        if let Some(expr) = self.include_deleted.clone() {
            r = r.filter(expr)
        }
        let r = r
            .filter(self.col.is_in(keys))
            ._gql_select(&self.look_ahead, self.col)?
            .all(tx)
            .await?;
        let mut map = HashMap::<String, E::G>::new();
        for g in r {
            map.insert(
                g._get_col(self.col)
                    .ok_or_else(|| MyErr::LoaderColumnValue)?,
                g,
            );
        }
        Ok(map)
    }
}
