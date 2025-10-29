use crate::prelude::*;

#[model]
pub struct User {
    pub email: String,
    #[graphql(skip)]
    pub password_hashed: String,
}
