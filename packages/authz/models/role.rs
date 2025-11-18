use crate::prelude::*;

#[model]
pub struct Role {
    pub key: String,
    /// Can be none if this role is not related to an org. For example system admin or built in roles.
    pub org_id: Option<String>,
}
