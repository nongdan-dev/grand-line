use crate::prelude::*;
use async_graphql::{Context, extensions::ExtensionContext};

pub trait ContextX {
    fn grand_line_context(&self) -> Arc<GrandLineContext>;
}

impl ContextX for Context<'_> {
    #[inline(always)]
    fn grand_line_context(&self) -> Arc<GrandLineContext> {
        self.data_unchecked::<Arc<GrandLineContext>>().clone()
    }
}

impl ContextX for ExtensionContext<'_> {
    #[inline(always)]
    fn grand_line_context(&self) -> Arc<GrandLineContext> {
        self.data_unchecked::<Arc<GrandLineContext>>().clone()
    }
}
