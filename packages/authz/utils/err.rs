use crate::prelude::*;

#[grand_line_err]
pub enum MyErr {
    // ========================================================================
    // client errors
    //
    #[error("unauthorized")]
    #[client]
    Unauthorized,
    #[error("org id is missing in the request headers")]
    #[client]
    HeaderOrgId404,
    // ========================================================================
    // server errors
    //
    #[error("authz requires macro call in the resolver definition")]
    MissingMacro,
}
