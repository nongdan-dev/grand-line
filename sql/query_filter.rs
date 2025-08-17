use crate::prelude::*;

/// Abstract extra QueryFilter methods implementation.
pub trait QueryFilterX<T, M, A, F, O, G>
where
    T: EntityX<M, A, F, O, G>,
    M: FromQueryResult + Send + Sync,
    A: ActiveModelTrait<Entity = T>,
    F: Filter<T>,
    O: OrderBy<T>,
    G: FromQueryResult + Send + Sync,
    Self: QueryFilter,
{
    /// Filter with condition deleted_at is not null, if there is deleted_at.
    fn include_deleted(self, include_deleted: Option<bool>) -> Self {
        match T::cond_include_deleted(include_deleted) {
            Some(c) => self.filter(c),
            None => self,
        }
    }
}

/// Automatically implement for Select<T>.
impl<T, M, A, F, O, G> QueryFilterX<T, M, A, F, O, G> for Select<T>
where
    T: EntityX<M, A, F, O, G>,
    M: FromQueryResult + Send + Sync,
    A: ActiveModelTrait<Entity = T>,
    F: Filter<T>,
    O: OrderBy<T>,
    G: FromQueryResult + Send + Sync,
{
}
/// Automatically implement for DeleteMany<T>.
impl<T, M, A, F, O, G> QueryFilterX<T, M, A, F, O, G> for DeleteMany<T>
where
    T: EntityX<M, A, F, O, G>,
    M: FromQueryResult + Send + Sync,
    A: ActiveModelTrait<Entity = T>,
    F: Filter<T>,
    O: OrderBy<T>,
    G: FromQueryResult + Send + Sync,
{
}
/// Automatically implement for UpdateMany<T>.
impl<T, M, A, F, O, G> QueryFilterX<T, M, A, F, O, G> for UpdateMany<T>
where
    T: EntityX<M, A, F, O, G>,
    M: FromQueryResult + Send + Sync,
    A: ActiveModelTrait<Entity = T>,
    F: Filter<T>,
    O: OrderBy<T>,
    G: FromQueryResult + Send + Sync,
{
}

/// Abstract extra QueryFilter methods implementation.
pub(crate) trait QueryFilterXInternal<T, M, A, F, O, G>
where
    T: EntityX<M, A, F, O, G>,
    M: FromQueryResult + Send + Sync,
    A: ActiveModelTrait<Entity = T>,
    F: Filter<T>,
    O: OrderBy<T>,
    G: FromQueryResult + Send + Sync,
    Self: QueryFilter,
{
    /// Filter with condition id eq.
    fn by_id(self, id: &str) -> Res<Self> {
        T::cond_id(id).map(|c| self.filter(c))
    }
}

/// Automatically implement for Select<T>.
impl<T, M, A, F, O, G> QueryFilterXInternal<T, M, A, F, O, G> for Select<T>
where
    T: EntityX<M, A, F, O, G>,
    M: FromQueryResult + Send + Sync,
    A: ActiveModelTrait<Entity = T>,
    F: Filter<T>,
    O: OrderBy<T>,
    G: FromQueryResult + Send + Sync,
{
}
/// Automatically implement for DeleteMany<T>.
impl<T, M, A, F, O, G> QueryFilterXInternal<T, M, A, F, O, G> for DeleteMany<T>
where
    T: EntityX<M, A, F, O, G>,
    M: FromQueryResult + Send + Sync,
    A: ActiveModelTrait<Entity = T>,
    F: Filter<T>,
    O: OrderBy<T>,
    G: FromQueryResult + Send + Sync,
{
}
/// Automatically implement for UpdateMany<T>.
impl<T, M, A, F, O, G> QueryFilterXInternal<T, M, A, F, O, G> for UpdateMany<T>
where
    T: EntityX<M, A, F, O, G>,
    M: FromQueryResult + Send + Sync,
    A: ActiveModelTrait<Entity = T>,
    F: Filter<T>,
    O: OrderBy<T>,
    G: FromQueryResult + Send + Sync,
{
}
