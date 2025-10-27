use super::prelude::*;

/// Abstract gql model methods implementation.
pub trait GqlModel<E>
where
    E: EntityX<G = Self>,
    Self: FromQueryResult + Default + Clone + Send + Sync + Sized,
{
    fn _set_id(self, v: &str) -> Self;
    fn _get_col(&self, col: E::C) -> Option<String>;
}
