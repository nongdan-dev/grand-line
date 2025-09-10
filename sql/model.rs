use crate::prelude::*;

/// Abstract extra model methods implementation.
pub trait ModelX<T>
where
    T: EntityX,
    Self: FromQueryResult + IntoActiveModel<T::A> + Send + Sync + Sized,
{
    fn _get_id(&self) -> String;
    // Convert sql model to gql model, without checking virtual fields from context.
    fn _to_gql(self) -> T::G;
}
