use super::prelude::*;

pub trait GrandLineContext<'a> {
    fn grand_line(&self) -> Res<&'a GrandLineContextData>;
}

impl<'a> GrandLineContext<'a> for Context<'a> {
    fn grand_line(&self) -> Res<&'a GrandLineContextData> {
        let gl = self
            .data_opt::<Arc<GrandLineContextData>>()
            .ok_or(MyErr::Ctx404)?;
        Ok(gl)
    }
}

impl<'a> GrandLineContext<'a> for ExtensionContext<'a> {
    fn grand_line(&self) -> Res<&'a GrandLineContextData> {
        let gl = self
            .data_opt::<Arc<GrandLineContextData>>()
            .ok_or(MyErr::Ctx404)?;
        Ok(gl)
    }
}
