use crate::prelude::*;

// Per-request cache for auth result.
pub struct AuthCache(pub Option<LoginSessionSql>);

#[async_trait]
pub trait AuthCacheContext<'a>
where
    Self: AuthHttpContext<'a>,
{
    async fn auth_unchecked(&self) -> Res<Arc<AuthCache>> {
        let arc = self.cache(|| self.auth_unchecked_without_cache()).await?;
        Ok(arc)
    }

    async fn auth_unchecked_without_cache(&self) -> Res<AuthCache> {
        let mut t = self.get_authorization_token()?;
        if t.is_empty() {
            t = self.get_cookie_login_session()?;
        }

        let t = rand_utils::qs_token_parse(&t);
        let Some(t) = t else {
            return Ok(AuthCache(None));
        };

        let lsd = self.login_session_data()?;
        let tx = &*self.tx().await?;

        let ls = LoginSession::find()
            .exclude_deleted()
            .filter_by_id(&t.id)
            .one(tx)
            .await?;
        let Some(ls) = ls else {
            return Ok(AuthCache(None));
        };

        if !rand_utils::secret_eq(&ls.secret_hashed, &t.secret) {
            return Ok(AuthCache(None));
        }

        let c = self.auth_config();
        if ls.created_at < now() - duration_ms(c.cookie_login_session_expires_ms) {
            return Ok(AuthCache(None));
        }

        let ls = am_update!(LoginSession {
            id: ls.id,
            ip: lsd.ip,
            ua: lsd.ua.to_json()?,
        })
        .exec_without_ctx(tx)
        .await?;

        Ok(AuthCache(Some(ls)))
    }
}

#[async_trait]
impl<'a> AuthCacheContext<'a> for Context<'a> {
}
