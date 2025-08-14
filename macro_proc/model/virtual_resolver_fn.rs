use crate::prelude::*;

pub trait VirtualResolverFn: ResolverFn {
    fn sql_deps(&self) -> Vec<String>;
}
