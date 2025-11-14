use crate::prelude::*;

#[search(LoginSession, no_include_deleted, auth=authenticate)]
fn resolver() {
    let f = get_filter(ctx).await?;
    let o = order_by_some!(LoginSession[UpdatedAtDesc]);
    (f, o)
}

#[count(LoginSession, no_include_deleted, auth=authenticate)]
fn resolver() {
    get_filter(ctx).await?
}

async fn get_filter(ctx: &Context<'_>) -> Res<Option<LoginSessionFilter>> {
    let ls = ctx.authenticate().await?;
    let f = filter_some!(LoginSession {
        id_ne: ls.id,
        user_id: ls.user_id,
    });
    Ok(f)
}
