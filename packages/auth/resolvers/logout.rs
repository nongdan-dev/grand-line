use crate::prelude::*;

#[mutation(auth)]
async fn logout() -> LoginSessionGql {
    let arc = ctx.auth_arc().await?;
    let ls = arc.as_ref().as_ref().ok_or(MyErr::Unauthenticated)?;

    LoginSession::delete_by_id(&ls.id).exec(tx).await?;

    LoginSessionGql::from_id(&ls.id)
}
