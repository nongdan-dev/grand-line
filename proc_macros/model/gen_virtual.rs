use crate::prelude::*;

pub trait GenVirtual
where
    Self: GenResolver,
{
    fn sql_dep(&self) -> String;
}
