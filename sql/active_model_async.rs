use crate::prelude::*;

/// Abstract extra active model async methods implementation.
#[async_trait]
pub trait ActiveModelXAsync<T>
where
    T: EntityX,
    Self: ActiveModelX<T>,
{
    // Set delete_at and update db.
    async fn soft_delete<D>(self, db: &D) -> Res<T::M>
    where
        D: ConnectionTrait,
        T::M: IntoActiveModel<Self>,
    {
        let r = self._delete().update(db).await?;
        Ok(r)
    }
}

/// Automatically implement for ActiveModelXAsync<T>.
#[async_trait]
impl<T, A> ActiveModelXAsync<T> for A
where
    T: EntityX,
    A: ActiveModelX<T>,
{
}
