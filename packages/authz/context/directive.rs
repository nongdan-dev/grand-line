use crate::prelude::*;

#[gql_enum]
pub enum AuthzDirectiveCheck {
    Org,
    User,
}

pub struct AuthzDirectiveEnsure {
    pub scope: String,
    pub org: bool,
    pub user: bool,
}

#[TypeDirective(name = "authz", location = "FieldDefinition")]
pub fn authz_directive(scope: String, check: Vec<AuthzDirectiveCheck>) {}
