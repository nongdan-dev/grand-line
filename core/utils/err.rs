use crate::prelude::*;

#[grand_line_err]
pub enum MyErr {
    // ========================================================================
    // server errors
    //
    #[error("json error: {inner}")]
    Json {
        #[from]
        inner: JsonErr,
    },
}

impl From<JsonErr> for GqlErr {
    fn from(v: JsonErr) -> Self {
        MyErr::from(v).into()
    }
}
