use crate::prelude::*;

pub const AUTH_TICKET_REGISTER: &str = "register";
pub const AUTH_TICKET_FORGOT: &str = "forgot";

#[model]
pub struct AuthTicket {
    pub ty: String,
    /// register, forgot
    pub email: String,
    /// register
    pub password_hashed: String,
    /// register, forgot
    pub otp: String,
}
