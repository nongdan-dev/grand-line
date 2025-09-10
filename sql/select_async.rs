use crate::prelude::*;

/// Abstract extra Select async methods implementation.
#[async_trait]
pub trait SelectXAsync<T>
where
    T: EntityX,
{
    /// Helper to check exists.
    async fn exists<D>(self, db: &D) -> Res<bool>
    where
        D: ConnectionTrait;

    /// Helper to check if exists and return error if not.
    async fn try_exists<D>(self, db: &D) -> Res<()>
    where
        D: ConnectionTrait;
}

/// Automatically implement for Select<T>.
#[async_trait]
impl<T> SelectXAsync<T> for Select<T>
where
    T: EntityX,
{
    /// Helper to check exists.
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

    /// Helper to check if exists and return error if not.
    async fn try_exists<D>(self, db: &D) -> Res<()>
    where
        D: ConnectionTrait,
    {
        match self.exists(db).await? {
            true => Ok(()),
            false => err_client!(Db404),
        }
    }
}

/// Abstract extra Select async methods implementation.
/// Make it simpler to also implement for Selector<SelectModel<G>>.
#[async_trait]
pub trait SelectXAsync2<G>
where
    G: FromQueryResult + Send + Sync,
{
    /// Helper to find one and return error if not.
    async fn try_one<D>(self, db: &D) -> Res<G>
    where
        D: ConnectionTrait;
}

/// Automatically implement for Select<T>.
#[async_trait]
impl<T> SelectXAsync2<T::M> for Select<T>
where
    T: EntityX,
{
    /// Helper to find one and return error if not.
    async fn try_one<D>(self, db: &D) -> Res<T::M>
    where
        D: ConnectionTrait,
    {
        match self.one(db).await? {
            Some(v) => Ok(v),
            None => err_client!(Db404),
        }
    }
}

/// Automatically implement for Selector<SelectModel<G>>.
#[async_trait]
impl<G> SelectXAsync2<G> for Selector<SelectModel<G>>
where
    G: FromQueryResult + Send + Sync,
{
    /// Helper to find one and return error if not.
    async fn try_one<D>(self, db: &D) -> Res<G>
    where
        D: ConnectionTrait,
    {
        match self.one(db).await? {
            Some(v) => Ok(v),
            None => err_client!(Db404),
        }
    }
}
