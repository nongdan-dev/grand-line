use super::prelude::*;

#[async_trait]
pub trait AuthenticateAsyncContext {
    async fn _authenticate_without_cache(&self) -> Res<Option<LoginSessionSql>>;
    async fn _authenticate(&self) -> Res<Arc<Option<LoginSessionSql>>>;
    async fn authenticate_optional(&self) -> Res<Option<LoginSessionSql>>;
    async fn authenticate(&self) -> Res<LoginSessionSql>;
    async fn _ensure_authenticated(&self) -> Res<()>;
    async fn _ensure_not_authenticated(&self) -> Res<()>;
}

#[async_trait]
impl AuthenticateAsyncContext for Context<'_> {
    async fn _authenticate_without_cache(&self) -> Res<Option<LoginSessionSql>> {
        let mut token = self._header_authorization()?;
        if token.is_empty() {
            token = self._cookie_login_session()?;
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

        if ls.secret != t.secret {
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

    async fn _authenticate(&self) -> Res<Arc<Option<LoginSessionSql>>> {
        let arc = self.get_cache::<Option<LoginSessionSql>>().await?;
        if let Some(arc) = arc {
            return Ok(arc);
        }
        let ls = self._authenticate_without_cache().await?;
        let arc = self.cache(ls).await?;
        Ok(arc)
    }

    async fn authenticate_optional(&self) -> Res<Option<LoginSessionSql>> {
        let ls = self._authenticate().await?.as_ref().as_ref().cloned();
        Ok(ls)
    }

    async fn authenticate(&self) -> Res<LoginSessionSql> {
        let ls = self
            ._authenticate()
            .await?
            .as_ref()
            .as_ref()
            .cloned()
            .ok_or(MyErr::Unauthenticated)?;
        Ok(ls)
    }

    async fn _ensure_authenticated(&self) -> Res<()> {
        if self._authenticate().await?.as_ref().is_none() {
            err!(Unauthenticated)?;
        }
        Ok(())
    }

    async fn _ensure_not_authenticated(&self) -> Res<()> {
        if self._authenticate().await?.as_ref().is_some() {
            err!(AlreadyAuthenticated)?;
        }
        Ok(())
    }
}
