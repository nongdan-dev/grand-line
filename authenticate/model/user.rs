use crate::prelude::*;

#[model]
pub struct User {
    pub email: String,
    #[graphql(skip)]
    pub hashed_password: String,
}
