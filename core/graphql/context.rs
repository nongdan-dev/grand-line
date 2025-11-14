use super::prelude::*;

pub trait GrandLineContext<'a> {
    fn grand_line(&self) -> Res<&'a GrandLineState>;
}

impl<'a> GrandLineContext<'a> for Context<'a> {
    fn grand_line(&self) -> Res<&'a GrandLineState> {
        let gl = self
            .data_opt::<Arc<GrandLineState>>()
            .ok_or(MyErr::Ctx404)?;
        Ok(gl)
    }
}

impl<'a> GrandLineContext<'a> for ExtensionContext<'a> {
    fn grand_line(&self) -> Res<&'a GrandLineState> {
        let gl = self
            .data_opt::<Arc<GrandLineState>>()
            .ok_or(MyErr::Ctx404)?;
        Ok(gl)
    }
}
