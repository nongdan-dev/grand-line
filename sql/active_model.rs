use crate::prelude::*;

pub trait ActiveModelX<T>
where
    T: EntityX,
    Self: ActiveModelTrait<Entity = T> + ActiveModelBehavior,
{
}
