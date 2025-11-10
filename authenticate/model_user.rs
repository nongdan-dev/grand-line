use super::prelude::*;

#[model(no_by_id)]
pub struct User {
    pub email: String,
    #[graphql(skip)]
    pub password_hashed: String,
}
