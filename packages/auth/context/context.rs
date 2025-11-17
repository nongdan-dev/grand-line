use crate::prelude::*;

#[async_trait]
pub trait AuthContext {
    async fn auth(&self) -> Res<String>;
    async fn auth_without_cache(&self) -> Res<Option<LoginSessionSql>>;
    async fn auth_arc(&self) -> Res<Arc<Option<LoginSessionSql>>>;
    async fn ensure_authenticated(&self) -> Res<()>;
    async fn ensure_not_authenticated(&self) -> Res<()>;
    async fn ensure_auth_in_macro(&self, v: AuthEnsure) -> Res<()>;
    fn get_cookie_login_session(&self) -> Res<String>;
    fn set_cookie_login_session(&self, ls: &LoginSessionSql) -> Res<()>;
}

#[async_trait]
impl AuthContext for Context<'_> {
    async fn auth(&self) -> Res<String> {
        let user_id = self
            .auth_arc()
            .await?
            .as_ref()
            .as_ref()
            .map(|ls| ls.user_id.clone())
            .ok_or(MyErr::Unauthenticated)?;
        Ok(user_id)
    }

    async fn auth_without_cache(&self) -> Res<Option<LoginSessionSql>> {
        let mut token = self.get_authorization_token()?;
        if token.is_empty() {
            token = self.get_cookie_login_session()?;
        }

        let t = rand_utils::qs_token_parse(&token);
        let t = if let Some(t) = t {
            t
        } else {
            return Ok(None);
        };

        let lsd = login_session_ensure_data(self)?;
        let tx = &*self.tx().await?;

        let ls = LoginSession::find_by_id(&t.id).one(tx).await?;
        let ls = if let Some(ls) = ls {
            ls
        } else {
            return Ok(None);
        };

        if !rand_utils::constant_time_eq(&ls.secret, &t.secret) {
            return Ok(None);
        }

        let ls = am_update!(LoginSession {
            id: ls.id,
            ua: lsd.ua.to_json()?,
            ip: lsd.ip,
        })
        .update(tx)
        .await?;

        Ok(Some(ls))
    }

    async fn auth_arc(&self) -> Res<Arc<Option<LoginSessionSql>>> {
        let arc = self.cache(|| self.auth_without_cache()).await?;
        Ok(arc)
    }

    async fn ensure_authenticated(&self) -> Res<()> {
        if self.auth_arc().await?.as_ref().is_none() {
            Err(MyErr::Unauthenticated)?;
        }
        Ok(())
    }

    async fn ensure_not_authenticated(&self) -> Res<()> {
        if self.auth_arc().await?.as_ref().is_some() {
            Err(MyErr::AlreadyAuthenticated)?;
        }
        Ok(())
    }

    async fn ensure_auth_in_macro(&self, v: AuthEnsure) -> Res<()> {
        match v {
            AuthEnsure::None => {}
            AuthEnsure::Authenticate => self.ensure_authenticated().await?,
            AuthEnsure::Unauthenticated => self.ensure_not_authenticated().await?,
        }
        Ok(())
    }

    fn get_cookie_login_session(&self) -> Res<String> {
        let c = &self.auth_config();
        let v = self
            .get_cookie(c.cookie_login_session_key)?
            .unwrap_or_default();
        Ok(v)
    }

    fn set_cookie_login_session(&self, ls: &LoginSessionSql) -> Res<()> {
        let c = &self.auth_config();
        let token = rand_utils::qs_token(&ls.id, &ls.secret)?;
        self.set_cookie(
            c.cookie_login_session_key,
            &token,
            c.cookie_login_session_expires,
        );
        Ok(())
    }
}
