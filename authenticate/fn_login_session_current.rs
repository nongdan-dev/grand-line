use super::prelude::*;

#[query]
async fn loginSessionCurrent() -> Option<LoginSessionGql> {
    if let Some(ls) = ctx.authenticate_opt().await? {
        Some(ls.into_gql(ctx).await?)
    } else {
        None
    }
}
