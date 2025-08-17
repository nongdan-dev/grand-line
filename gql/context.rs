use crate::prelude::*;

pub trait ContextX
where
    Self: GrandLineContextImpl,
{
}

impl ContextX for Context<'_> {}
