use crate::prelude::*;

pub trait GenVirtual
where
    Self: GenResolverFn,
{
    fn sql_dep(&self) -> String;
}
