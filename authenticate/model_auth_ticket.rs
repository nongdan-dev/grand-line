use super::prelude::*;

#[model]
pub struct AuthTicket {
    pub ty: AuthTicketTy,

    #[graphql(skip)]
    pub email: String,

    #[default(random_otp_6digits())]
    #[graphql(skip)]
    pub otp: String,

    #[default(random_secret_256bit())]
    pub secret: String,

    #[graphql(skip)]
    pub data: JsonValue,
}

#[enunn]
pub enum AuthTicketTy {
    Register,
    Forgot,
}

#[derive(Serialize, Deserialize)]
pub struct AuthTicketDataRegister {
    pub password_hashed: String,
}

#[derive(Serialize, Deserialize)]
pub struct AuthTicketDataForgot {
    pub user_id: String,
}
