use super::prelude::*;

pub trait GrandLineExtensionContext {
    fn _grand_line_context(&self) -> Res<Arc<GrandLineContext>>;
}

impl GrandLineExtensionContext for Context<'_> {
    #[inline(always)]
    fn _grand_line_context(&self) -> Res<Arc<GrandLineContext>> {
        map_err(self.data::<Arc<GrandLineContext>>())
    }
}

impl GrandLineExtensionContext for ExtensionContext<'_> {
    #[inline(always)]
    fn _grand_line_context(&self) -> Res<Arc<GrandLineContext>> {
        map_err(self.data::<Arc<GrandLineContext>>())
    }
}

fn map_err(r: Result<&Arc<GrandLineContext>, GraphQLErr>) -> Res<Arc<GrandLineContext>> {
    let a = r.cloned().map_err(|e| MyErr::Ctx404 { inner: e.message })?;
    Ok(a)
}
