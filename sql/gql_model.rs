use crate::prelude::*;

/// Abstract gql model methods implementation.
pub trait GqlModel<E>
where
    E: EntityX<G = Self>,
    Self: FromQueryResult + Default + Send + Sync + Sized,
{
    fn _set_id(self, v: &str) -> Self;
}
