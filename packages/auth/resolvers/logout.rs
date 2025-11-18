use crate::prelude::*;

#[mutation(auth = "authenticated")]
async fn logout() -> LoginSessionGql {
    let arc = ctx.auth_with_cache().await?;
    let ls = arc.as_ref().as_ref().ok_or(MyErr::Unauthenticated)?;

    LoginSession::delete_by_id(&ls.id).exec(tx).await?;

    LoginSessionGql::from_id(&ls.id)
}
