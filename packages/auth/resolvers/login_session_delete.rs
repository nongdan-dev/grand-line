use crate::prelude::*;

pub async fn login_session_delete_impl(ctx: &Context<'_>, id: String) -> Res<LoginSessionGql> {
    let tx = &*ctx.tx().await?;
    let arc = ctx.auth_with_cache().await?;
    let ls = arc.as_ref().as_ref().ok_or(MyErr::Unauthenticated)?;

    LoginSession::delete_by_id(&id)
        .filter(LoginSessionColumn::UserId.eq(&ls.user_id))
        .exec(tx)
        .await?;

    Ok(LoginSessionGql::from_id(&id))
}

pub async fn login_session_delete_all_impl(ctx: &Context<'_>) -> Res<Vec<LoginSessionGql>> {
    let tx = &*ctx.tx().await?;
    let arc = ctx.auth_with_cache().await?;
    let ls = arc.as_ref().as_ref().ok_or(MyErr::Unauthenticated)?;

    let r = LoginSession::find()
        .exclude_deleted()
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

    Ok(r)
}
