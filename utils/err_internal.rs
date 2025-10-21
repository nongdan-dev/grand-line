use crate::prelude::*;

#[grand_line_err]
pub enum MyErr {
    // ========================================================================
    // CLIENT ERRORS
    // CAN BE EXPOSED TO THE CLIENT
    // ========================================================================
    //
    /// Generic code for all server errors to prevent exposing server error.
    #[error("internal server error")]
    #[client]
    InternalServer,

    #[error("not found in database")]
    #[client]
    Db404,

    #[error("failed to get request ip address")]
    #[client]
    CtxReqIp404,
    #[error("failed to get request user agent")]
    #[client]
    CtxReqUa404,

    // ========================================================================
    // SERVER ERROR
    // WILL NOT BE EXPOSED TO THE CLIENT
    // ========================================================================
    //
    #[error("database error: {inner}")]
    Db {
        #[from]
        inner: DbErr,
    },
    #[error("`{field}` column is not present in the model `{model}`")]
    DbCfgField404 {
        model: &'static str,
        field: &'static str,
    },
    #[error("`{field}` have no value in the active model `{model}`")]
    DbAmField404 {
        model: &'static str,
        field: &'static str,
    },

    #[error("json error: {inner}")]
    Json {
        #[from]
        inner: JsonErr,
    },

    #[error("no grand line context in the async graphql context: {inner}")]
    Ctx404 { inner: String },
    #[error("no sea orm database in the async graphql context: {inner}")]
    CtxDb404 { inner: String },

    #[error("commit error: transaction is still in use elsewhere")]
    TxCommit,
    #[error("rollback error: transaction is still in use elsewhere")]
    TxRollback,
    #[error("look ahead selection fields len should be 1")]
    LookAhead,

    #[error("no request headers in the async graphql context: {inner}")]
    CtxReqHeaders404 { inner: String },

    #[error("FRAMEWORK BUG: id column is not present in the model {model}")]
    BugId404 { model: &'static str },
}
pub type GrandLineInternalErr = MyErr;

impl From<MyErr> for ServerError {
    fn from(v: MyErr) -> Self {
        GrandLineErr(Arc::new(v)).into()
    }
}
impl From<DbErr> for GrandLineErr {
    fn from(v: DbErr) -> Self {
        MyErr::from(v).into()
    }
}
impl From<JsonErr> for GrandLineErr {
    fn from(v: JsonErr) -> Self {
        MyErr::from(v).into()
    }
}
