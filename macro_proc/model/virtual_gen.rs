use crate::prelude::*;

pub trait VirtualGen
where
    Self: GenResolverFn,
{
    fn sql_deps(&self) -> Vec<String>;
}
