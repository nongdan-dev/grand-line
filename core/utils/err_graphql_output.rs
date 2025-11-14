use crate::prelude::*;
use std::fmt::{Formatter, Result as FmtResult};

#[derive(Clone)]
pub struct GqlErr(pub Arc<dyn GqlErrImpl>);
pub type Res<T> = Result<T, GqlErr>;

impl Display for GqlErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(&self.0, f)
    }
}
impl Debug for GqlErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Debug::fmt(&self.0, f)
    }
}
impl Error for GqlErr {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.0.source()
    }
}

impl From<GqlErr> for ServerError {
    fn from(v: GqlErr) -> Self {
        let mut e = Self::new(v.to_string(), None);
        e.source = Some(Arc::new(v));
        e
    }
}
