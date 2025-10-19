use crate::prelude::*;

#[derive(ThisError, Debug)]
pub enum GrandLineErrorServer {
    #[error("database error: {0}")]
    Db(#[from] DbErr),
    #[error("{0} column is not present in the model {1}")]
    DbCfgF404(&'static str, &'static str),
    #[error("{0} have no value in the active model {1}")]
    DbAmF404(&'static str, &'static str),

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
}

#[derive(ThisError, Debug)]
pub enum GrandLineErrorClient {
    #[error("404 data not found")]
    Db404,

    #[error("failed to get request ip address")]
    CtxReqIp404,
    #[error("failed to get request user agent")]
    CtxReqUa404,
}

#[derive(ThisError, Debug)]
pub enum GrandLineError {
    #[error(transparent)]
    Server(#[from] GrandLineErrorServer),
    #[error(transparent)]
    Client(#[from] GrandLineErrorClient),
}

impl From<DbErr> for GrandLineError {
    fn from(e: DbErr) -> Self {
        GrandLineErrorServer::Db(e).into()
    }
}

pub type GrandLineResult<T> = Result<T, GrandLineError>;
pub(crate) use macro_utils::{err_client, err_client_res, err_server, err_server_res};
pub(crate) type Res<T> = GrandLineResult<T>;
