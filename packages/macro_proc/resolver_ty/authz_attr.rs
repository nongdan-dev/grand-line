use crate::prelude::*;

#[field_names]
pub struct AuthzAttr {
    pub key: String,
    pub org: bool,
    pub user: bool,
}
