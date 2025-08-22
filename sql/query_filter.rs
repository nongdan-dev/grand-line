use crate::prelude::*;

/// Abstract extra QueryFilter methods implementation.
pub trait QueryFilterX<T>
where
    T: EntityX,
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
impl<T> QueryFilterX<T> for Select<T> where T: EntityX {}
/// Automatically implement for DeleteMany<T>.
impl<T> QueryFilterX<T> for DeleteMany<T> where T: EntityX {}
/// Automatically implement for UpdateMany<T>.
impl<T> QueryFilterX<T> for UpdateMany<T> where T: EntityX {}

/// Abstract extra QueryFilter methods implementation, internal only.
pub(crate) trait QueryFilterXInternal<T>
where
    T: EntityX,
    Self: QueryFilter,
{
    /// Filter with condition id eq.
    fn by_id(self, id: &str) -> Res<Self> {
        T::cond_id(id).map(|c| self.filter(c))
    }
}

/// Automatically implement for Select<T>.
impl<T> QueryFilterXInternal<T> for Select<T> where T: EntityX {}
/// Automatically implement for DeleteMany<T>.
impl<T> QueryFilterXInternal<T> for DeleteMany<T> where T: EntityX {}
/// Automatically implement for UpdateMany<T>.
impl<T> QueryFilterXInternal<T> for UpdateMany<T> where T: EntityX {}
