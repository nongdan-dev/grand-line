use super::prelude::*;

pub trait CoreContext<'a>
where
    Self: ImplContext<'a>
        + GrandLineDataContext<'a>
        + CacheContext<'a>
        + CoreConfigContext<'a>
        + TxContext<'a>
        + DataLoaderContext<'a>,
{
}

impl<'a> CoreContext<'a> for Context<'a> {
}
