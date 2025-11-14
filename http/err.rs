use super::prelude::*;

#[grand_line_err]
pub enum MyErr {
    // ========================================================================
    // client errors
    //
    #[error("request header {k} has more than 1 value")]
    #[client]
    MultipleHeaderValues { k: String },
    #[error("ip address is missing from the request")]
    #[client]
    Ip404,
    #[error("user agent is missing from the request")]
    #[client]
    Ua404,

    // ========================================================================
    // server errors
    //
    #[error("context missing request headers")]
    CtxReqHeaders404,
}
