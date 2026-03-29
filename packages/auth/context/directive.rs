use crate::prelude::*;

#[gql_enum]
pub enum AuthDirectiveCheck {
    Authenticated,
    Unauthenticated,
}

#[TypeDirective(name = "auth", location = "FieldDefinition")]
pub fn auth_directive(check: AuthDirectiveCheck) {}
