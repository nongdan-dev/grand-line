use super::prelude::*;
use zxcvbn::{Score, zxcvbn};

#[derive(Clone)]
pub struct AuthConfig {
    pub default_ensure: GqlAuthEnsure,
    pub cookie_login_session_key: &'static str,
    pub cookie_login_session_expires: i64,
    pub otp_max_attempt: i64,
    pub otp_expire_ms: i64,
    pub otp_re_request_ms: i64,
    pub handlers: Arc<dyn AuthHandlers>,
}

impl Default for AuthConfig {
    fn default() -> Self {
        Self {
            default_ensure: GqlAuthEnsure::None,
            cookie_login_session_key: "login_session",
            cookie_login_session_expires: 7 * 24 * 60 * 60 * 1000,
            otp_max_attempt: 5,
            otp_expire_ms: 10 * 60 * 1000,
            otp_re_request_ms: 60 * 1000,
            handlers: Arc::new(DefaultAuthHandlers),
        }
    }
}

#[derive(Clone)]
pub enum GqlAuthEnsure {
    None,
    Authenticate,
    Unauthenticated,
}

#[async_trait]
pub trait AuthHandlers
where
    Self: Send + Sync,
{
    async fn validate_password(&self, _ctx: &Context<'_>, password: &str) -> Res<()> {
        if zxcvbn(password, &[]).score() < Score::Three {
            Err(MyErr::PasswordInvalid)?;
        }
        Ok(())
    }
    async fn otp(&self, _ctx: &Context<'_>) -> Res<String> {
        Ok(otp_new())
    }
    async fn on_otp_create(
        &self,
        _ctx: &Context<'_>,
        _otp: &AuthOtpSql,
        _otp_raw: &str,
    ) -> Res<()> {
        Ok(())
    }
    async fn on_register_resolve(&self, _ctx: &Context<'_>, _user: &UserSql) -> Res<()> {
        Ok(())
    }
    async fn on_login_resolve(&self, _ctx: &Context<'_>, _user: &UserSql) -> Res<()> {
        Ok(())
    }
    async fn on_forgot_resolve(&self, _ctx: &Context<'_>, _user: &UserSql) -> Res<()> {
        Ok(())
    }
}

struct DefaultAuthHandlers;
#[async_trait]
impl AuthHandlers for DefaultAuthHandlers {}
