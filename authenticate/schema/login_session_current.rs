use crate::prelude::*;

#[query]
async fn loginSessionCurrent() -> Option<LoginSessionGql> {
    let mut token = ctx.get_header_authorization()?;
    if token.is_empty() {
        token = ctx.get_cookie_login_session()?;
    }
    if let Some(t) = qs_token_parse(&token)
        && let Some(ls) = LoginSession::find_by_id(&t.id).one(tx).await?
        && ls.secret == t.secret
    {
        let ls = db_update!(
            tx,
            LoginSession {
                ip: ctx.get_ip()?,
                ua: ctx.get_ua()?,
                ..ls.into_active_model()
            }
        );
        Some(ls.into_gql(ctx).await?)
    } else {
        None
    }
}
