use crate::prelude::*;

#[query]
async fn login_session_current() -> Option<LoginSessionGql> {
    if let Some(ls) = ctx.auth_unchecked().await?.as_ref().0.as_ref() {
        Some(ls.clone().into_gql(ctx).await?)
    } else {
        None
    }
}
