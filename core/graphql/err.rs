use super::prelude::*;

#[grand_line_err]
pub enum MyErr {
    #[error("internal server error")]
    #[client]
    InternalServer,

    #[error("ip address is missing from the request")]
    #[client]
    CtxReqIp404,
    #[error("user agent is missing from the request")]
    #[client]
    CtxReqUa404,

    #[error("commit error: transaction is still in use elsewhere")]
    TxCommit,
    #[error("rollback error: transaction is still in use elsewhere")]
    TxRollback,

    #[error("context has no grand line context: {inner}")]
    Ctx404 { inner: String },
    #[error("context has no sea orm database: {inner}")]
    CtxDb404 { inner: String },
}
pub type GrandLineInternalGraphQLErr = MyErr;

impl From<MyErr> for ServerError {
    fn from(v: MyErr) -> Self {
        GrandLineErr(Arc::new(v)).into()
    }
}
