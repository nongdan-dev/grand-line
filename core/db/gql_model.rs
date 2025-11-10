use super::prelude::*;

/// Abstract gql model methods implementation.
pub trait GqlModel<E>
where
    E: EntityX<G = Self>,
    Self: FromQueryResult + Default + Clone + Send + Sync,
{
    /// Should be generated in the model macro.
    fn set_id(self, v: &str) -> Self;
    /// Should be generated in the model macro.
    fn get_string(&self, col: E::C) -> Option<String>;
}
