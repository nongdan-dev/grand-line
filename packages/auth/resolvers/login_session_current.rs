use crate::prelude::*;

pub async fn login_session_current_impl(ctx: &Context<'_>) -> Res<Option<LoginSessionGql>> {
    if let Some(ls) = ctx.auth_with_cache().await?.as_ref().as_ref() {
        Ok(Some(ls.clone().into_gql(ctx).await?))
    } else {
        Ok(None)
    }
}
