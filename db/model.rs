use super::prelude::*;

/// Abstract extra model methods implementation.
pub trait ModelX<E>
where
    E: EntityX<M = Self>,
    Self: FromQueryResult + IntoActiveModel<E::A> + Send + Sync + Sized,
{
    fn _get_id(&self) -> String;

    // Convert sql model to gql model, without checking virtual fields from context.
    fn _to_gql(self) -> E::G;
}
