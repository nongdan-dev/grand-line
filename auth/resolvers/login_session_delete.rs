use crate::prelude::*;

#[mutation(auth=authenticate)]
fn login_session_delete(id: String) -> LoginSessionGql {
    let ls = ctx.authenticate().await?;

    LoginSession::delete_by_id(&id)
        .filter(LoginSessionColumn::UserId.eq(&ls.user_id))
        .exec(tx)
        .await?;

    LoginSessionGql::from_id(&ls.id)
}

#[mutation(auth=authenticate)]
fn login_session_delete_all() -> Vec<LoginSessionGql> {
    let ls = ctx.authenticate().await?;

    let r = LoginSession::find()
        .filter(LoginSessionColumn::Id.ne(&ls.id))
        .filter(LoginSessionColumn::UserId.eq(&ls.user_id))
        .gql_select_id()
        .all(tx)
        .await?;

    LoginSession::delete_many()
        .filter(LoginSessionColumn::Id.ne(&ls.id))
        .filter(LoginSessionColumn::UserId.eq(&ls.user_id))
        .exec(tx)
        .await?;

    r
}
