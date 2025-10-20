use std::fmt::{Debug, Formatter, Result as FmtResult};

use crate::prelude::*;

pub trait GrandLineErrImpl
where
    Self: Error + Send + Sync,
{
    fn code(&self) -> &'static str;
    fn client(&self) -> bool;
    fn client_code(&self) -> &'static str {
        if self.client() {
            self.code()
        } else {
            MyErr::ServerError.code()
        }
    }
}

#[derive(Clone)]
pub struct GrandLineErr(pub Arc<dyn GrandLineErrImpl>);
impl Display for GrandLineErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(&self.0, f)
    }
}
impl Debug for GrandLineErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Debug::fmt(&self.0, f)
    }
}
impl Error for GrandLineErr {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.0.source()
    }
}
impl From<GrandLineErr> for ErrorExtensionValues {
    fn from(v: GrandLineErr) -> Self {
        let mut m = ErrorExtensionValues::default();
        m.set("code", v.0.client_code());
        m
    }
}

pub type Res<T> = Result<T, GrandLineErr>;
