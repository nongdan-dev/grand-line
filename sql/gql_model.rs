use crate::prelude::*;

pub trait GqlModel<T>
where
    T: EntityX,
    Self: FromQueryResult,
{
}
