use super::prelude::*;

#[mutation]
async fn logout() -> LoginSessionGql {
    ctx.ensure_authenticated().await?;

    let ls = ctx.authenticate().await?;
    LoginSession::delete_by_id(&ls.id).exec(tx).await?;

    LoginSessionGql::from_id(&ls.id)
}
