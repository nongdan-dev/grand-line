use crate::prelude::*;

#[gql_enum]
pub enum AuthzDirectiveCheck {
    Org,
    User,
}

pub struct AuthzDirectiveEnsure {
    pub org: bool,
    pub user: bool,
    pub key: Option<String>,
}

#[TypeDirective(name = "authz", location = "FieldDefinition")]
pub fn authz_directive(check: Vec<AuthzDirectiveCheck>, key: Option<String>) {}
