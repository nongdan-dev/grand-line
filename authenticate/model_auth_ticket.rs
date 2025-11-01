use super::prelude::*;

#[model]
pub struct AuthTicket {
    pub ty: AuthTicketTy,
    pub email: String,
    #[default(random_otp_6digits())]
    pub otp: String,
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
