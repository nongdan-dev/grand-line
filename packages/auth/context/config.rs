use crate::prelude::*;

#[derive(Clone)]
pub struct AuthConfig {
    pub cookie_login_session_key: &'static str,
    pub cookie_login_session_expires_ms: i64,
    pub otp_max_attempt: i64,
    pub otp_expires_ms: i64,
    pub otp_re_request_ms: i64,
    pub handlers: Arc<dyn AuthHandlers>,
}

impl Default for AuthConfig {
    fn default() -> Self {
        Self {
            cookie_login_session_key: COOKIE_LOGIN_SESSION_KEY,
            cookie_login_session_expires_ms: COOKIE_LOGIN_SESSION_EXPIRES,
            otp_max_attempt: OTP_MAX_ATTEMPT,
            otp_expires_ms: OTP_EXPIRE_MS,
            otp_re_request_ms: OTP_RE_REQUEST_MS,
            handlers: Arc::new(DefaultHandlers),
        }
    }
}

#[allow(unused_variables)]
#[async_trait]
pub trait AuthHandlers
where
    Self: Send + Sync,
{
    async fn password_validate(&self, ctx: &Context<'_>, password: &str) -> Res<bool> {
        let ok = rand_utils::password_validate(password).is_ok();
        Ok(ok)
    }

    async fn otp(&self, ctx: &Context<'_>) -> Res<String> {
        let otp = rand_utils::otp();
        Ok(otp)
    }
    async fn on_otp_create(&self, ctx: &Context<'_>, otp: &AuthOtpSql, otp_raw: &str) -> Res<()> {
        Ok(())
    }

    async fn on_register_resolve(
        &self,
        ctx: &Context<'_>,
        user: &UserSql,
        ls: &LoginSessionSql,
    ) -> Res<()> {
        Ok(())
    }
    async fn on_login_resolve(
        &self,
        ctx: &Context<'_>,
        user: &UserSql,
        ls: &LoginSessionSql,
    ) -> Res<()> {
        Ok(())
    }
    async fn on_forgot_resolve(
        &self,
        ctx: &Context<'_>,
        user: &UserSql,
        ls: &LoginSessionSql,
    ) -> Res<()> {
        Ok(())
    }
}

struct DefaultHandlers;
#[async_trait]
impl AuthHandlers for DefaultHandlers {}
