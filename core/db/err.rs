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
    #[error("`{field}` have no value in the gql model `{model}`")]
    DbGqlField404 {
        model: &'static str,
        field: &'static str,
    },

    #[error("look ahead selection fields len should be 1")]
    LookAhead,
    #[error("data loader failed to downcast from arc dyn any")]
    LoaderDowncast,
    #[error("data loader failed to get value from column in gql model")]
    LoaderColumnValue,

    #[error("FRAMEWORK BUG: id column is not present in the model {model}")]
    BugId404 { model: &'static str },
}
pub type GrandLineInternalDbErr = MyErr;

impl From<DbErr> for GrandLineErr {
    fn from(v: DbErr) -> Self {
        MyErr::from(v).into()
    }
}
