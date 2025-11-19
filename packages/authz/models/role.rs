use crate::prelude::*;

#[model]
pub struct Role {
    pub name: String,
    /// To group multiple roles into a group. For example: system admin, public...
    /// It is optional, if no key is passed to the macro then it will not filter by this column.
    pub key: Option<String>,
    pub operations: JsonValue,
    /// Can be none if this role is not related to an org. For example: built in roles...
    pub org_id: Option<String>,
}
