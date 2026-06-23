use grand_line::prelude::*;

#[model]
pub struct Org {
    pub name: String,
    #[default("")]
    pub description: String,
}

impl AuthzOrg for Org {
}
