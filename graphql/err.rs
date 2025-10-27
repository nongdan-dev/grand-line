use super::prelude::*;

#[grand_line_err]
pub enum MyErr {
    #[error("internal server error")]
    #[client]
    InternalServer,

    #[error("failed to get request ip address")]
    #[client]
    CtxReqIp404,
    #[error("failed to get request user agent")]
    #[client]
    CtxReqUa404,

    #[error("commit error: transaction is still in use elsewhere")]
    TxCommit,
    #[error("rollback error: transaction is still in use elsewhere")]
    TxRollback,

    #[error("no grand line context in the async graphql context: {inner}")]
    Ctx404 { inner: String },
    #[error("no sea orm database in the async graphql context: {inner}")]
    CtxDb404 { inner: String },
    #[error("no request headers in the async graphql context: {inner}")]
    CtxReqHeaders404 { inner: String },

    #[error("look ahead selection fields len should be 1")]
    LookAhead,
    #[error("failed to downcast data loader from arc dyn any")]
    LoaderDowncast,
    #[error("failed to get value from column in gql model for data loader")]
    LoaderColumnValue,
}
pub type GrandLineInternalGraphQLErr = MyErr;

impl From<MyErr> for ServerError {
    fn from(v: MyErr) -> Self {
        GrandLineErr(Arc::new(v)).into()
    }
}
