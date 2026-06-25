use super::prelude::*;

pub trait GrandLineDataContext<'a>
where
    Self: BaseImplContext<'a>,
{
    fn grand_line(&self) -> Res<&'a GrandLineData> {
        let gl = self.data_opt_impl::<Arc<GrandLineData>>().ok_or(MyErr::Ctx404)?;
        Ok(gl)
    }
}

impl<'a> GrandLineDataContext<'a> for Context<'a> {
}

impl<'a> GrandLineDataContext<'a> for ExtensionContext<'a> {
}
