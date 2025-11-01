use crate::prelude::*;

#[grand_line_err]
pub enum MyErr {
    #[error("json error: {inner}")]
    Json {
        #[from]
        inner: JsonErr,
    },
}

impl From<JsonErr> for GrandLineErr {
    fn from(v: JsonErr) -> Self {
        MyErr::from(v).into()
    }
}
