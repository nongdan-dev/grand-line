use super::prelude::*;

#[search(LoginSession)]
fn resolver() {
    ctx.ensure_authenticated().await?;

    let f = get_filter(ctx).await?;
    let o = order_by_some!(LoginSession[UpdatedAtDesc]);
    (f, o)
}

#[count(LoginSession)]
fn resolver() {
    ctx.ensure_authenticated().await?;

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
