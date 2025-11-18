use crate::prelude::*;

#[gql_enum]
pub enum AuthDirectiveRule {
    Authenticated,
    Unauthenticated,
}

#[TypeDirective(name = "auth", location = "FieldDefinition")]
pub fn auth_directive(rule: AuthDirectiveRule) {}
