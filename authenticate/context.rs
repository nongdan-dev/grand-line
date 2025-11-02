use super::prelude::*;

pub trait GrandLineAuthenticateContext {
    fn get_cookie_login_session(&self) -> Res<String>;
    fn set_cookie_login_session(&self, ls: &LoginSessionSql) -> Res<()>;
}

impl GrandLineAuthenticateContext for Context<'_> {
    fn get_cookie_login_session(&self) -> Res<String> {
        let GrandLineConfigAuth {
            cookie_login_session_key: k,
            ..
        } = self.config().auth;
        let v = self.get_cookie(k)?.unwrap_or_default();
        Ok(v)
    }

    fn set_cookie_login_session(&self, ls: &LoginSessionSql) -> Res<()> {
        let GrandLineConfigAuth {
            cookie_login_session_key: k,
            cookie_login_session_expires: e,
            ..
        } = self.config().auth;
        let token = qs_token(&ls.id, &ls.secret)?;
        self.set_cookie(k, &token, e);
        Ok(())
    }
}
