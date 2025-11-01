use super::prelude::*;

/// Abstract extra QueryFilter methods implementation.
pub trait QueryFilterX<E>
where
    E: EntityX,
    Self: QueryFilter,
{
    /// Filter with condition deleted_at is not null, if there is deleted_at.
    fn include_deleted(self, include_deleted: Option<bool>) -> Self {
        match E::cond_deleted_at(include_deleted) {
            Some(c) => self.filter(c),
            None => self,
        }
    }
}

/// Automatically implement for Select<E>.
impl<E> QueryFilterX<E> for Select<E> where E: EntityX {}
/// Automatically implement for DeleteMany<E>.
impl<E> QueryFilterX<E> for DeleteMany<E> where E: EntityX {}
/// Automatically implement for UpdateMany<E>.
impl<E> QueryFilterX<E> for UpdateMany<E> where E: EntityX {}

/// Abstract extra QueryFilter methods implementation, internal only.
pub(crate) trait QueryFilterXInternal<E>
where
    E: EntityX,
    Self: QueryFilter,
{
    /// Filter with condition id eq.
    fn by_id(self, id: &str) -> Self {
        self.filter(E::cond_id(id))
    }
}

/// Automatically implement for Select<E>.
impl<E> QueryFilterXInternal<E> for Select<E> where E: EntityX {}
/// Automatically implement for DeleteMany<E>.
impl<E> QueryFilterXInternal<E> for DeleteMany<E> where E: EntityX {}
/// Automatically implement for UpdateMany<E>.
impl<E> QueryFilterXInternal<E> for UpdateMany<E> where E: EntityX {}
