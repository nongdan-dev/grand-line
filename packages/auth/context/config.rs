use crate::prelude::*;

/// Non-generic config: timeouts, keys, and non-user-model handlers.
/// Add this to your schema with `.data(AuthConfig::default())`.
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
pub trait AuthHandlers: Send + Sync {
    async fn password_validate(&self, ctx: &Context<'_>, password: &str) -> Res<bool> {
        Ok(rand_utils::password_validate(password).is_ok())
    }

    async fn otp(&self, ctx: &Context<'_>) -> Res<String> {
        Ok(rand_utils::otp())
    }

    async fn on_otp_create(&self, ctx: &Context<'_>, otp: &AuthOtpSql, otp_raw: &str) -> Res<()> {
        Ok(())
    }
}

struct DefaultHandlers;
#[async_trait]
impl AuthHandlers for DefaultHandlers {}

/// Generic user config: callbacks that receive the user's own model type.
/// Add this to your schema with `.data(AuthUserConfig::<User>::default())`.
pub struct AuthUserConfig<U: AuthUser> {
    pub handlers: Arc<dyn AuthUserHandlers<U>>,
}

impl<U: AuthUser> Default for AuthUserConfig<U> {
    fn default() -> Self {
        Self {
            handlers: Arc::new(DefaultUserHandlers(PhantomData)),
        }
    }
}

#[allow(unused_variables)]
#[async_trait]
pub trait AuthUserHandlers<U: AuthUser>: Send + Sync {
    async fn on_register_resolve(
        &self,
        ctx: &Context<'_>,
        user: &U::M,
        ls: &LoginSessionSql,
    ) -> Res<()> {
        Ok(())
    }

    async fn on_login_resolve(
        &self,
        ctx: &Context<'_>,
        user: &U::M,
        ls: &LoginSessionSql,
    ) -> Res<()> {
        Ok(())
    }

    async fn on_forgot_resolve(
        &self,
        ctx: &Context<'_>,
        user: &U::M,
        ls: &LoginSessionSql,
    ) -> Res<()> {
        Ok(())
    }
}

struct DefaultUserHandlers<U: AuthUser>(PhantomData<U>);
#[async_trait]
impl<U: AuthUser> AuthUserHandlers<U> for DefaultUserHandlers<U> {}
