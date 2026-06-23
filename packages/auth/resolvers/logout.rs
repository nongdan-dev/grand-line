use crate::prelude::*;

pub async fn logout_impl(ctx: &Context<'_>) -> Res<LoginSessionGql> {
    let tx = &*ctx.tx().await?;
    let arc = ctx.auth_with_cache().await?;
    let ls = arc.as_ref().as_ref().ok_or(MyErr::Unauthenticated)?;

    LoginSession::delete_by_id(&ls.id).exec(tx).await?;

    Ok(LoginSessionGql::from_id(&ls.id))
}
