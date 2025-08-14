use crate::prelude::*;

#[derive(ThisError, Debug)]
pub enum ErrServer {
    #[error("database error: {0}")]
    Db(#[from] DbErr),
    #[error("{0} is not present in active model")]
    DbAmField404(String),

    #[error("failed to get grand line context: {0}")]
    Ctx404(String),
    #[error("commit error: transaction is still in use elsewhere")]
    TxCommit,
    #[error("rollback error: transaction is still in use elsewhere")]
    TxRollback,
    #[error("look ahead selection fields list length should be 1")]
    LookAhead,

    #[error("server error: {0}")]
    New(String),

    #[error("FRAMEWORK BUG: id is not found in the model columns")]
    BugId404,
}

#[derive(ThisError, Debug)]
pub enum ErrClient {
    #[error("404 data not found")]
    Db404,

    #[error("{0}")]
    New(String),
}

#[derive(ThisError, Debug)]
pub enum GrandLineError {
    #[error(transparent)]
    Server(#[from] ErrServer),
    #[error(transparent)]
    Client(#[from] ErrClient),
}

impl From<DbErr> for GrandLineError {
    fn from(e: DbErr) -> Self {
        ErrServer::Db(e).into()
    }
}

pub type GrandLineResult<T> = Result<T, GrandLineError>;
pub(crate) use macro_utils::{err_client, err_server};
pub(crate) type Res<T> = GrandLineResult<T>;
