use super::prelude::*;

#[query]
async fn loginSessionCurrent() -> Option<LoginSessionGql> {
    if let Some(s) = ctx.authenticate_optional().await? {
        Some(s.into_gql(ctx).await?)
    } else {
        None
    }
}
