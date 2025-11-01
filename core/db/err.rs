use super::prelude::*;

#[grand_line_err]
pub enum MyErr {
    #[error("data not found")]
    #[client]
    Db404,

    #[error("database error: {inner}")]
    Db {
        #[from]
        inner: DbErr,
    },
    #[error("`{col}` column not found")]
    DbCol404 { col: String },
    #[error("resolver try to unwrap with no value")]
    GqlResolverNone,

    #[error("look ahead selection fields len should be 1")]
    LookAhead,
    #[error("data loader cannot downcast from arc dyn any")]
    LoaderDowncast,
    #[error("data loader cannot get string key with column `{col}`")]
    LoaderKeyNone { col: String },
}

impl From<DbErr> for GrandLineErr {
    fn from(v: DbErr) -> Self {
        MyErr::from(v).into()
    }
}
