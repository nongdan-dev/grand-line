use super::prelude::*;

pub trait GrandLineExtensionContext<'a> {
    fn grand_line_context(&self) -> Res<&'a GrandLineState>;
}

impl<'a> GrandLineExtensionContext<'a> for Context<'a> {
    fn grand_line_context(&self) -> Res<&'a GrandLineState> {
        let gl = self
            .data_opt::<Arc<GrandLineState>>()
            .ok_or(MyErr::Ctx404)?;
        Ok(gl)
    }
}

impl<'a> GrandLineExtensionContext<'a> for ExtensionContext<'a> {
    fn grand_line_context(&self) -> Res<&'a GrandLineState> {
        let gl = self
            .data_opt::<Arc<GrandLineState>>()
            .ok_or(MyErr::Ctx404)?;
        Ok(gl)
    }
}
