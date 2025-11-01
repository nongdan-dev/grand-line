use super::prelude::*;

/// Abstract extra model methods implementation.
pub trait ModelX<E>
where
    E: EntityX<M = Self>,
    Self: FromQueryResult + IntoActiveModel<E::A> + Send + Sync + Sized,
{
    /// Should be generated in the model macro.
    fn get_id(&self) -> String;
    /// Convert sql model to gql model, without checking virtual fields from context.
    /// Should be generated in the model macro.
    fn into_gql_without_look_ahead(self) -> E::G;
}
