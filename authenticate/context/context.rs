use crate::prelude::*;

const AUTHORIZATION: &str = "authorization";
const BEARER: &str = "Bearer ";
const LOGIN_SESSION: &str = "login_session";

pub trait AuthenticateContext {
    fn get_header_authorization(&self) -> Res<String>;
    fn get_cookie_login_session(&self) -> Res<String>;
    fn set_cookie_login_session(&self, v: &str);
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

    fn set_cookie_login_session(&self, v: &str) {
        self.insert_http_header(LOGIN_SESSION, v);
    }
}
