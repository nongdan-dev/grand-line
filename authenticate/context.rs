use super::prelude::*;

const AUTHORIZATION: &str = "Authorization";
const BEARER: &str = "Bearer ";
const LOGIN_SESSION: &str = "login_session";
const LOGIN_SESSION_EXPIRES: i64 = 30 * 24 * 60 * 60 * 1000;

pub trait AuthenticateContext {
    fn get_header_authorization(&self) -> Res<String>;
    fn get_cookie_login_session(&self) -> Res<String>;
    fn set_cookie_login_session(&self, ls: &LoginSessionSql) -> Res<()>;
}

impl AuthenticateContext for Context<'_> {
    fn get_header_authorization(&self) -> Res<String> {
        let v = self.get_header(AUTHORIZATION)?.replace(BEARER, "");
        Ok(v)
    }

    fn get_cookie_login_session(&self) -> Res<String> {
        let v = self.get_cookie(LOGIN_SESSION)?.unwrap_or_default();
        Ok(v)
    }

    fn set_cookie_login_session(&self, ls: &LoginSessionSql) -> Res<()> {
        let token = qs_token(&ls.id, &ls.secret)?;
        self.set_cookie(LOGIN_SESSION, &token, LOGIN_SESSION_EXPIRES);
        Ok(())
    }
}
