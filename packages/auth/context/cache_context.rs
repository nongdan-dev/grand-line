use crate::prelude::*;

#[async_trait]
pub trait AuthCacheContext {
    async fn auth_with_cache(&self) -> Res<Arc<Option<LoginSessionMinimal>>>;
    async fn auth_without_cache(&self) -> Res<Option<LoginSessionMinimal>>;
}

#[async_trait]
impl AuthCacheContext for Context<'_> {
    async fn auth_with_cache(&self) -> Res<Arc<Option<LoginSessionMinimal>>> {
        let arc = self.cache(|| self.auth_without_cache()).await?;
        Ok(arc)
    }

    async fn auth_without_cache(&self) -> Res<Option<LoginSessionMinimal>> {
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
        let ls = LoginSessionMinimal::select(q).one(tx).await?;
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
}
