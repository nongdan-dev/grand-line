use crate::prelude::*;

/// Abstract extra model methods implementation.
pub trait ModelX<T>
where
    T: EntityX,
    Self: FromQueryResult + IntoActiveModel<T::A> + Send + Sync + Sized,
{
    fn _to_gql(self) -> T::G;

    // Convert sql model into gql model, with virtual aware from context.
    fn to_gql(self, ctx: &Context<'_>) -> T::G {
        // TODO:
        self._to_gql()
    }
}
