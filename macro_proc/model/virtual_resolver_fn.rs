use crate::prelude::*;

pub trait VirtualResolverFn: ResolverFn {
    fn sql_dep(&self) -> Vec<String>;
}
