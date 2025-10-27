use super::prelude::*;

/// Abstract extra Select async methods implementation.
#[async_trait]
pub trait SelectXAsync<E>
where
    E: EntityX,
{
    /// Helper to check if exists.
    async fn exists<D>(self, db: &D) -> Res<bool>
    where
        D: ConnectionTrait;

    /// Helper to check if exists and return error if not.
    async fn try_exists<D>(self, db: &D) -> Res<()>
    where
        D: ConnectionTrait;
}

/// Automatically implement for Select<E>.
#[async_trait]
impl<E> SelectXAsync<E> for Select<E>
where
    E: EntityX,
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
            false => err!(Db404),
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

/// Automatically implement for Select<E>.
#[async_trait]
impl<E> SelectXAsync2<E::M> for Select<E>
where
    E: EntityX,
{
    async fn try_one<D>(self, db: &D) -> Res<E::M>
    where
        D: ConnectionTrait,
    {
        match self.one(db).await? {
            Some(v) => Ok(v),
            None => err!(Db404),
        }
    }
}

/// Automatically implement for Selector<SelectModel<G>>.
#[async_trait]
impl<G> SelectXAsync2<G> for Selector<SelectModel<G>>
where
    G: FromQueryResult + Send + Sync,
{
    async fn try_one<D>(self, db: &D) -> Res<G>
    where
        D: ConnectionTrait,
    {
        match self.one(db).await? {
            Some(v) => Ok(v),
            None => err!(Db404),
        }
    }
}
