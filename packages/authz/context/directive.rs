use crate::prelude::*;

#[gql_input]
pub struct AuthzRule {
    pub key: String,
    pub org: bool,
    pub user: bool,
}

#[TypeDirective(name = "authz", location = "FieldDefinition")]
pub fn authz_directive(rule: Option<AuthzRule>) {}
