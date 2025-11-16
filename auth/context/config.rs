use crate::prelude::*;

#[derive(Clone)]
pub struct AuthConfig {
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
            cookie_login_session_key: "login_session",
            cookie_login_session_expires: 7 * 24 * 60 * 60 * 1000,
            otp_max_attempt: 5,
            otp_expire_ms: 10 * 60 * 1000,
            otp_re_request_ms: 60 * 1000,
            handlers: Arc::new(DefaultAuthHandlers),
        }
    }
}

#[allow(unused_variables)]
#[async_trait]
pub trait AuthHandlers
where
    Self: Send + Sync,
{
    async fn otp(&self, ctx: &Context<'_>) -> Res<String> {
        let otp = auth_utils::otp();
        Ok(otp)
    }
    async fn password_validate(&self, ctx: &Context<'_>, password: &str) -> Res<()> {
        auth_utils::password_validate(password)?;
        Ok(())
    }
    async fn on_otp_create(&self, ctx: &Context<'_>, otp: &AuthOtpSql, otp_raw: &str) -> Res<()> {
        Ok(())
    }
    async fn on_register_resolve(&self, ctx: &Context<'_>, user: &UserSql) -> Res<()> {
        Ok(())
    }
    async fn on_login_resolve(&self, ctx: &Context<'_>, user: &UserSql) -> Res<()> {
        Ok(())
    }
    async fn on_forgot_resolve(&self, ctx: &Context<'_>, user: &UserSql) -> Res<()> {
        Ok(())
    }
}

struct DefaultAuthHandlers;
#[async_trait]
impl AuthHandlers for DefaultAuthHandlers {}
