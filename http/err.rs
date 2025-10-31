use super::prelude::*;

#[grand_line_err]
pub enum MyErr {
    #[error("ip address is missing from the request")]
    #[client]
    CtxReqIp404,
    #[error("user agent is missing from the request")]
    #[client]
    CtxReqUa404,

    #[error("context has no request headers: {inner}")]
    CtxReqHeaders404 { inner: String },
}
