use crate::prelude::*;

#[query]
async fn loginSessionCurrent() -> Option<LoginSessionGql> {
    let mut token = ctx.get_header_authorization()?;
    if token.is_empty() {
        token = ctx.get_cookie_login_session()?;
    }
    if let Some(t) = qs_token_parse(&token)
        && let Some(s) = LoginSession::find_by_id(&t.id).one(tx).await?
        && s.secret == t.secret
    {
        Some(s.into_gql(ctx).await?)
    } else {
        None
    }
}
