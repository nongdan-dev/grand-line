use super::prelude::*;

pub trait ContextGrandLineImpl {
    fn grand_line_context(&self) -> Res<Arc<GrandLineContext>>;
}

impl ContextGrandLineImpl for Context<'_> {
    #[inline(always)]
    fn grand_line_context(&self) -> Res<Arc<GrandLineContext>> {
        map_err(self.data::<Arc<GrandLineContext>>())
    }
}

impl ContextGrandLineImpl for ExtensionContext<'_> {
    #[inline(always)]
    fn grand_line_context(&self) -> Res<Arc<GrandLineContext>> {
        map_err(self.data::<Arc<GrandLineContext>>())
    }
}

fn map_err(r: Result<&Arc<GrandLineContext>, GraphQLErr>) -> Res<Arc<GrandLineContext>> {
    let a = r.cloned().map_err(|e| MyErr::Ctx404 { inner: e.message })?;
    Ok(a)
}
