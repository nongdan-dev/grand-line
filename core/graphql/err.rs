use super::prelude::*;

#[grand_line_err]
pub enum MyErr {
    #[error("internal server error")]
    #[client]
    InternalServer,

    #[error("commit error: transaction is still in use elsewhere")]
    TxCommit,
    #[error("rollback error: transaction is still in use elsewhere")]
    TxRollback,

    #[error("context has no grand line context: {inner}")]
    Ctx404 { inner: String },
    #[error("context has no sea orm database: {inner}")]
    CtxDb404 { inner: String },

    #[error("cache cannot downcast from once cell arc")]
    CacheDowncast,
}

impl From<MyErr> for ServerError {
    fn from(v: MyErr) -> Self {
        GrandLineErr(Arc::new(v)).into()
    }
}
