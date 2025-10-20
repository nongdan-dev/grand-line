use crate::prelude::*;

#[grand_line_err]
pub enum MyErr {
    // ========================================================================
    // SERVER ONLY, NOT EXPOSE TO CLIENT
    //
    #[error("database error: {0}")]
    Db(#[from] DbErr),
    #[error("{0} column is not present in the model {1}")]
    DbCfgField404(&'static str, &'static str),
    #[error("{0} have no value in the active model {1}")]
    DbAmField404(&'static str, &'static str),

    #[error("no grand line context in the async graphql context: {0}")]
    Ctx404(String),
    #[error("commit error: transaction is still in use elsewhere")]
    TxCommit,
    #[error("rollback error: transaction is still in use elsewhere")]
    TxRollback,
    #[error("look ahead selection fields length should be 1")]
    LookAhead,

    #[error("no request headers in the async graphql context: {0}")]
    CtxReqHeaders404(String),

    #[error("FRAMEWORK BUG: id column is not present in the model {0}")]
    BugId404(&'static str),

    // ========================================================================
    // EXPOSE TO CLIENT
    //
    /// generic code for all server errors
    #[error("server error")]
    #[client]
    ServerError,

    #[error("404 data not found")]
    #[client]
    Db404,

    #[error("failed to get request ip address")]
    #[client]
    CtxReqIp404,
    #[error("failed to get request user agent")]
    #[client]
    CtxReqUa404,
}

impl From<DbErr> for GrandLineErr {
    fn from(v: DbErr) -> Self {
        MyErr::from(v).into()
    }
}
