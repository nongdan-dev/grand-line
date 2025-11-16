use crate::prelude::*;

#[gql_enum]
pub enum AuthEnsure {
    None,
    Authenticate,
    Unauthenticated,
}

#[TypeDirective(name = "auth", location = "FieldDefinition")]
pub fn auth_directive(ensure: AuthEnsure) {}
