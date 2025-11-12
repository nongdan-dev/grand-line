use super::prelude::*;

#[async_trait]
pub trait GrandLineAuthenticateContextAsync {
    async fn authenticate_without_cache(&self) -> Res<Option<LoginSessionSql>>;
    async fn authenticate_arc(&self) -> Res<Arc<Option<LoginSessionSql>>>;
    async fn authenticate_opt(&self) -> Res<Option<LoginSessionSql>>;
    async fn authenticate(&self) -> Res<LoginSessionSql>;
    async fn ensure_authenticated(&self) -> Res<()>;
    async fn ensure_not_authenticated(&self) -> Res<()>;
}

#[async_trait]
impl GrandLineAuthenticateContextAsync for Context<'_> {
    async fn authenticate_without_cache(&self) -> Res<Option<LoginSessionSql>> {
        let mut token = self.get_authorization_token()?;
        if token.is_empty() {
            token = self.get_cookie_login_session()?;
        }

        let t = qs_token_parse(&token);
        let t = if let Some(t) = t {
            t
        } else {
            return Ok(None);
        };

        let tx = &*self.tx().await?;

        let ls = LoginSession::find_by_id(&t.id).one(tx).await?;
        let ls = if let Some(ls) = ls {
            ls
        } else {
            return Ok(None);
        };

        if !constant_time_eq(&ls.secret, &t.secret) {
            return Ok(None);
        }

        let ls = db_update!(
            tx,
            LoginSession {
                ip: self.get_ip()?,
                ua: self.get_ua()?,
                ..ls.into_active_model()
            }
        );
        Ok(Some(ls))
    }

    async fn authenticate_arc(&self) -> Res<Arc<Option<LoginSessionSql>>> {
        let arc = self.cache(|| self.authenticate_without_cache()).await?;
        Ok(arc)
    }

    async fn authenticate_opt(&self) -> Res<Option<LoginSessionSql>> {
        let ls = self.authenticate_arc().await?.as_ref().as_ref().cloned();
        Ok(ls)
    }

    async fn authenticate(&self) -> Res<LoginSessionSql> {
        let ls = self
            .authenticate_arc()
            .await?
            .as_ref()
            .as_ref()
            .cloned()
            .ok_or(MyErr::Unauthenticated)?;
        Ok(ls)
    }

    async fn ensure_authenticated(&self) -> Res<()> {
        if self.authenticate_arc().await?.as_ref().is_none() {
            Err(MyErr::Unauthenticated)?;
        }
        Ok(())
    }

    async fn ensure_not_authenticated(&self) -> Res<()> {
        if self.authenticate_arc().await?.as_ref().is_some() {
            Err(MyErr::AlreadyAuthenticated)?;
        }
        Ok(())
    }
}
