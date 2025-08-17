use crate::prelude::*;

/// Abstract extra Select async methods implementation.
#[async_trait]
pub trait SelectXAsync<T, M, A, F, O, G>
where
    T: EntityX<M, A, F, O, G> + 'static,
    M: FromQueryResult + Send + Sync + 'static,
    A: ActiveModelTrait<Entity = T> + 'static,
    F: Filter<T> + 'static,
    O: OrderBy<T> + 'static,
    G: FromQueryResult + Send + Sync + 'static,
    Self: QueryFilter + QuerySelect + 'static,
{
    /// Helper to check exists.
    async fn exists<D>(self, db: &D) -> Res<bool>
    where
        D: ConnectionTrait;
    /// Helper to check if exists and return error if not.
    async fn try_exists<D>(self, db: &D) -> Res<()>
    where
        D: ConnectionTrait;
    /// Helper to find one and return error if not.
    async fn try_one<D>(self, db: &D) -> Res<M>
    where
        D: ConnectionTrait;
}

/// Automatically implement for Select<T>.
#[async_trait]
impl<T, M, A, F, O, G> SelectXAsync<T, M, A, F, O, G> for Select<T>
where
    T: EntityX<M, A, F, O, G> + 'static,
    M: FromQueryResult + Send + Sync + 'static,
    A: ActiveModelTrait<Entity = T> + 'static,
    F: Filter<T> + 'static,
    O: OrderBy<T> + 'static,
    G: FromQueryResult + Send + Sync + 'static,
{
    async fn exists<D>(self, db: &D) -> Res<bool>
    where
        D: ConnectionTrait,
    {
        let v = self.select().expr(Expr::value(1)).limit(1).one(db).await?;
        match v {
            Some(_) => Ok(true),
            None => Ok(false),
        }
    }

    async fn try_exists<D>(self, db: &D) -> Res<()>
    where
        D: ConnectionTrait,
    {
        match self.exists(db).await? {
            true => Ok(()),
            false => err_client!(Db404),
        }
    }

    async fn try_one<D>(self, db: &D) -> Res<M>
    where
        D: ConnectionTrait,
    {
        match self.one(db).await? {
            Some(v) => Ok(v),
            None => err_client!(Db404),
        }
    }
}
