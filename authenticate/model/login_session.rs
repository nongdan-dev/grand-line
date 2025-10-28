use crate::prelude::*;

#[model]
pub struct LoginSession {
    pub secret: String,
    pub user_id: String,
    #[belongs_to]
    pub user: User,
}
