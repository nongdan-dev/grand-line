use sea_orm::*;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GrandLineError {
    #[error("database error: {0}")]
    Db(#[from] DbErr),
    #[error("404 data not found")]
    Db404,

    #[error("commit error: transaction is still in use elsewhere")]
    TxCommit,
    #[error("rollback error: transaction is still in use elsewhere")]
    TxRollback,
    #[error("look ahead selection fields must be 1")]
    LookAhead,
}

pub(crate) type Res<T> = Result<T, GrandLineError>;
