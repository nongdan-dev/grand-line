use super::prelude::*;

#[async_trait]
pub trait AuthenticateAsyncContext {
    async fn authenticate(&self) -> Res<Option<LoginSessionSql>>;
}

#[async_trait]
impl AuthenticateAsyncContext for Context<'_> {
    async fn authenticate(&self) -> Res<Option<LoginSessionSql>> {
        let _tx = self.tx().await?;
        let tx = _tx.as_ref();
        // TODO: cache lock mutex
        let mut token = self.get_header_authorization()?;
        if token.is_empty() {
            token = self.get_cookie_login_session()?;
        }
        let r = if let Some(t) = qs_token_parse(&token)
            && let Some(ls) = LoginSession::find_by_id(&t.id).one(tx).await?
            && ls.secret == t.secret
        {
            let ls = db_update!(
                tx,
                LoginSession {
                    ip: self.get_ip()?,
                    ua: self.get_ua()?,
                    ..ls.into_active_model()
                }
            );
            Some(ls)
        } else {
            None
        };
        Ok(r)
    }
}
