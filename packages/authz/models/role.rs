use crate::prelude::*;

#[model]
pub struct Role {
    pub name: String,
    /// To group multiple roles into a group. For example: system admin, public...
    pub key: String,
    pub operations: JsonValue,
    /// Can be none if this role is not related to an org. For example: built in roles...
    pub org_id: Option<String>,
}
