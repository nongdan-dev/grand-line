use crate::prelude::*;

pub trait VirtualResolverFn
where
    Self: ResolverFn,
{
    fn sql_deps(&self) -> Vec<String>;
}
