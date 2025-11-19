use crate::prelude::*;

#[async_trait]
pub trait AuthContext {
    async fn auth(&self) -> Res<String>;
    async fn auth_with_cache(&self) -> Res<Arc<Option<LoginSessionCache>>>;
    async fn auth_without_cache(&self) -> Res<Option<LoginSessionCache>>;
    async fn auth_ensure_authenticated(&self) -> Res<()>;
    async fn auth_ensure_not_authenticated(&self) -> Res<()>;
    async fn auth_ensure_in_macro(&self, check: AuthDirectiveCheck) -> Res<()>;
    fn get_cookie_login_session(&self) -> Res<String>;
    fn set_cookie_login_session(&self, ls: &LoginSessionSql) -> Res<()>;
}

#[async_trait]
impl AuthContext for Context<'_> {
    async fn auth(&self) -> Res<String> {
        let user_id = self
            .auth_with_cache()
            .await?
            .as_ref()
            .as_ref()
            .map(|ls| ls.user_id.clone())
            .ok_or(MyErr::Unauthenticated)?;
        Ok(user_id)
    }

    async fn auth_with_cache(&self) -> Res<Arc<Option<LoginSessionCache>>> {
        let arc = self.cache(|| self.auth_without_cache()).await?;
        Ok(arc)
    }

    async fn auth_without_cache(&self) -> Res<Option<LoginSessionCache>> {
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

        let q = LoginSession::find().exclude_deleted().filter_by_id(&t.id);

        let tx = &*self.tx().await?;
        let ls = LoginSessionCache::select(q).one(tx).await?;
        let ls = if let Some(ls) = ls {
            ls
        } else {
            return Ok(None);
        };

        if !rand_utils::constant_time_eq(&ls.secret, &t.secret) {
            return Ok(None);
        }

        am_update!(LoginSession {
            id: ls.id.clone(),
            ip: lsd.ip,
            ua: lsd.ua.to_json()?,
        })
        .update(tx)
        .await?;

        Ok(Some(ls))
    }

    async fn auth_ensure_authenticated(&self) -> Res<()> {
        if self.auth_with_cache().await?.as_ref().is_none() {
            Err(MyErr::Unauthenticated)?;
        }
        Ok(())
    }

    async fn auth_ensure_not_authenticated(&self) -> Res<()> {
        if self.auth_with_cache().await?.as_ref().is_some() {
            Err(MyErr::AlreadyAuthenticated)?;
        }
        Ok(())
    }

    async fn auth_ensure_in_macro(&self, check: AuthDirectiveCheck) -> Res<()> {
        match check {
            AuthDirectiveCheck::Authenticated => self.auth_ensure_authenticated().await?,
            AuthDirectiveCheck::Unauthenticated => self.auth_ensure_not_authenticated().await?,
        }
        Ok(())
    }

    fn get_cookie_login_session(&self) -> Res<String> {
        let c = &self.auth_config();
        let k = c.cookie_login_session_key;
        let v = self.get_cookie(k)?.unwrap_or_default();
        Ok(v)
    }

    fn set_cookie_login_session(&self, ls: &LoginSessionSql) -> Res<()> {
        let c = &self.auth_config();
        let k = c.cookie_login_session_key;
        let expires = c.cookie_login_session_expires;
        let token = rand_utils::qs_token(&ls.id, &ls.secret)?;
        self.set_cookie(k, &token, expires);
        Ok(())
    }
}

#[derive(FromQueryResult)]
pub struct LoginSessionCache {
    pub id: String,
    pub secret: String,
    pub user_id: String,
}
impl LoginSessionCache {
    pub fn select(q: Select<LoginSession>) -> Selector<SelectModel<Self>> {
        q.select_only()
            .column(LoginSessionColumn::Id)
            .column(LoginSessionColumn::Secret)
            .column(LoginSessionColumn::UserId)
            .into_model::<LoginSessionCache>()
    }
}
