use crate::prelude::*;

#[query]
async fn login_session_current() -> Option<LoginSessionGql> {
    if let Some(ls) = ctx.auth_with_cache().await?.as_ref().as_ref() {
        let ls = LoginSession::find()
            .exclude_deleted()
            .filter_by_id(&ls.id)
            .gql_select(ctx)?
            .one_or_404(tx)
            .await?;
        Some(ls)
    } else {
        None
    }
}
