use super::prelude::*;

#[grand_line_err]
pub enum MyErr {
    // ========================================================================
    // client errors
    //
    #[error("request header {k} has more than 1 value")]
    #[client]
    HeaderMultipleValues { k: String },
    #[error("ip address is missing in the request headers")]
    #[client]
    HeaderIp404,
    #[error("user agent is missing in the request headers")]
    #[client]
    HeaderUa404,

    // ========================================================================
    // server errors
    //
    #[error("context missing request headers")]
    CtxHeaders404,
}
