use super::prelude::*;

/// Abstract extra active model async methods implementation.
#[async_trait]
pub trait ActiveModelXAsync<E>
where
    E: EntityX<A = Self>,
    Self: ActiveModelX<E>,
{
    /// Set deleted_at and update db.
    /// It also checks if the model has configured with deleted_at column or not.
    async fn soft_delete<D>(self, db: &D) -> Res<E::M>
    where
        D: ConnectionTrait,
    {
        E::check_col_deleted_at()?;
        let r = self.set_defaults_on_delete().update(db).await?;
        Ok(r)
    }
}

/// Automatically implement for ActiveModelXAsync<E>.
#[async_trait]
impl<E, A> ActiveModelXAsync<E> for A
where
    E: EntityX<A = A>,
    A: ActiveModelX<E>,
{
}
