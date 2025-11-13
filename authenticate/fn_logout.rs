use super::prelude::*;

#[mutation(auth=authenticate)]
async fn logout() -> LoginSessionGql {
    let ls = ctx.authenticate().await?;
    LoginSession::delete_by_id(&ls.id).exec(tx).await?;

    LoginSessionGql::from_id(&ls.id)
}
