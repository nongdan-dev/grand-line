use crate::prelude::*;

/// Abstract gql model methods implementation.
pub trait GqlModel<T>
where
    T: EntityX,
    Self: FromQueryResult + Default + Send + Sync + Sized,
{
    fn _set_id(self, v: &str) -> Self;
}
