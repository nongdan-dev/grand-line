use crate::prelude::*;

pub trait VirtualGen
where
    Self: GenResolverFn,
{
    fn sql_dep(&self) -> Vec<String>;
}
