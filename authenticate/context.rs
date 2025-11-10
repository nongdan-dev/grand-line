use super::prelude::*;

pub trait GrandLineAuthenticateContext {
    fn get_cookie_login_session(&self) -> Res<String>;
    fn set_cookie_login_session(&self, ls: &LoginSessionSql) -> Res<()>;
}

impl GrandLineAuthenticateContext for Context<'_> {
    fn get_cookie_login_session(&self) -> Res<String> {
        let c = &self.config().auth;
        let v = self
            .get_cookie(c.cookie_login_session_key)?
            .unwrap_or_default();
        Ok(v)
    }

    fn set_cookie_login_session(&self, ls: &LoginSessionSql) -> Res<()> {
        let c = &self.config().auth;
        let token = qs_token(&ls.id, &ls.secret)?;
        self.set_cookie(
            c.cookie_login_session_key,
            &token,
            c.cookie_login_session_expires,
        );
        Ok(())
    }
}
