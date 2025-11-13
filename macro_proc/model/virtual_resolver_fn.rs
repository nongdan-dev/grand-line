use crate::prelude::*;

pub trait VirtualResolverFn
where
    Self: ResolverFn,
{
    fn sql_dep(&self) -> Vec<String>;
}
