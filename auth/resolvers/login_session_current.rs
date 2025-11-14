use crate::prelude::*;

#[query(auth=none)]
async fn login_session_current() -> Option<LoginSessionGql> {
    if let Some(ls) = ctx.authenticate_opt().await? {
        Some(ls.into_gql(ctx).await?)
    } else {
        None
    }
}
