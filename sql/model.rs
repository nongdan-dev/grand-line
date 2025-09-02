use crate::prelude::*;

pub trait ModelX<T>
where
    T: EntityX,
    Self: FromQueryResult + IntoActiveModel<T::A>,
{
}
