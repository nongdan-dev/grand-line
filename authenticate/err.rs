use crate::prelude::*;
use argon2::password_hash::Error as PasswordHashErr;
use serde_qs::Error as QsErr;

#[grand_line_err]
pub enum MyErr {
    // ========================================================================
    // client errors
    //
    #[error("unauthenticated")]
    #[client]
    Unauthenticated,
    #[error("already authenticated")]
    #[client]
    AlreadyAuthenticated,

    #[error("this email address is already in use")]
    #[client]
    RegisterEmailExists,

    #[error("password is too weak or invalid")]
    #[client]
    PasswordInvalid,

    #[error("otp is expired or invalid")]
    #[client]
    OtpResolveInvalid,
    #[error("otp is not yet to re-request")]
    #[client]
    OtpReRequestTooSoon,

    #[error("email or password is incorrect")]
    #[client]
    LoginIncorrect,

    // ========================================================================
    // server errors
    //
    #[error("hash password error: {inner}")]
    PasswordHash { inner: PasswordHashErr },
    #[error("query string error: {inner}")]
    QsErr { inner: QsErr },
    #[error("hmac error: {inner}")]
    HmacErr { inner: String },
}

impl From<PasswordHashErr> for MyErr {
    fn from(v: PasswordHashErr) -> Self {
        MyErr::PasswordHash { inner: v }
    }
}

impl From<QsErr> for MyErr {
    fn from(v: QsErr) -> Self {
        MyErr::QsErr { inner: v }
    }
}
