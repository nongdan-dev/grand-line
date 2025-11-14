use super::prelude::*;

pub trait GqlContext<'a> {
    fn grand_line(&self) -> Res<&'a GqlContextData>;
}

impl<'a> GqlContext<'a> for Context<'a> {
    fn grand_line(&self) -> Res<&'a GqlContextData> {
        let gl = self
            .data_opt::<Arc<GqlContextData>>()
            .ok_or(MyErr::Ctx404)?;
        Ok(gl)
    }
}

impl<'a> GqlContext<'a> for ExtensionContext<'a> {
    fn grand_line(&self) -> Res<&'a GqlContextData> {
        let gl = self
            .data_opt::<Arc<GqlContextData>>()
            .ok_or(MyErr::Ctx404)?;
        Ok(gl)
    }
}
