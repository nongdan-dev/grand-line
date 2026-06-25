use crate::prelude::*;

#[mutation(auth)]
async fn logout() -> LoginSessionGql {
    ctx.auth_ensure_authenticated().await?;

    let tx = &*ctx.tx().await?;
    let arc = ctx.auth_unchecked().await?;
    let ls = arc.as_ref().0.as_ref().ok_or(MyErr::Unauthenticated)?;

    LoginSession::delete_by_id(&ls.id).exec(tx).await?;

    LoginSessionGql::from_id(&ls.id)
}
