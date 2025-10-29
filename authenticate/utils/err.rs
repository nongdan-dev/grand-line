use crate::prelude::*;
use argon2::password_hash::Error as PasswordHashErr;
use serde_qs::Error as QsErr;

#[grand_line_err]
pub enum MyErr {
    #[error("hash password error: {inner}")]
    PasswordHash { inner: PasswordHashErr },
    #[error("query string error: {inner}")]
    QsErr { inner: QsErr },

    #[error("this email address is already in use")]
    RegisterEmailExists,
    #[error("invalid or expired otp")]
    OtpResolveInvalid,

    #[error("email or password is incorrect")]
    LoginIncorrect,
}
pub type GrandLineInternalAuthenticateErr = MyErr;

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
