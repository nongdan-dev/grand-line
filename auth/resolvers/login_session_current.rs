use crate::prelude::*;

#[query]
async fn login_session_current() -> Option<LoginSessionGql> {
    if let Some(ls) = ctx.auth_arc().await?.as_ref().as_ref() {
        Some(ls.clone().into_gql(ctx).await?)
    } else {
        None
    }
}
