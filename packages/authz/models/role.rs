use crate::prelude::*;

#[model]
pub struct Role {
    pub name: String,
    /// To group multiple roles into a realm. For example: system, org, public...
    pub realm: String,
    pub operations: JsonValue,
    /// Can be none if this role is not related to an org. For example system or built in roles.
    pub org_id: Option<String>,
}
