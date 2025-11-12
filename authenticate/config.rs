use super::prelude::*;
use zxcvbn::{Score::Three as ValidPasswordScore, zxcvbn};

pub struct GrandLineAuthConfig {
    pub default_ensure: GrandLineAuthConfigEnsure,
    pub cookie_login_session_key: &'static str,
    pub cookie_login_session_expires: i64,
    pub otp_max_attempt: i64,
    pub otp_expire_ms: i64,
    pub otp_resend_ms: i64,
    pub handlers: Arc<dyn GrandLineAuthHandlers>,
}

impl Default for GrandLineAuthConfig {
    fn default() -> Self {
        Self {
            default_ensure: GrandLineAuthConfigEnsure::Authenticate,
            cookie_login_session_key: "login_session",
            cookie_login_session_expires: 7 * 24 * 60 * 60 * 1000,
            otp_max_attempt: 5,
            otp_expire_ms: 10 * 60 * 1000,
            otp_resend_ms: 60 * 1000,
            handlers: Arc::new(DefaultAuthHandlers),
        }
    }
}

pub enum GrandLineAuthConfigEnsure {
    None,
    Authenticate,
    Unauthenticated,
    Authorize,
}

#[async_trait]
pub trait GrandLineAuthHandlers
where
    Self: Send + Sync,
{
    async fn validate_password(&self, _: &Context<'_>, password: &str) -> Res<()> {
        if zxcvbn(password, &[]).score() < ValidPasswordScore {
            Err(MyErr::PasswordInvalid)?;
        }
        Ok(())
    }
    async fn on_otp_create(&self, _: &Context<'_>, _: &AuthOtpSql) -> Res<()> {
        Ok(())
    }
    async fn on_register_resolve(&self, _: &Context<'_>, _: &UserSql) -> Res<()> {
        Ok(())
    }
    async fn on_login_resolve(&self, _: &Context<'_>, _: &UserSql) -> Res<()> {
        Ok(())
    }
    async fn on_forgot_resolve(&self, _: &Context<'_>, _: &UserSql) -> Res<()> {
        Ok(())
    }
}

struct DefaultAuthHandlers;
#[async_trait]
impl GrandLineAuthHandlers for DefaultAuthHandlers {}
