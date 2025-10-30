use crate::prelude::*;

#[model]
pub struct LoginSession {
    #[default(random_secret_256bit())]
    pub secret: String,
    pub user_id: String,
    #[belongs_to]
    pub user: User,
    pub ip: String,
    pub ua: String,
}
