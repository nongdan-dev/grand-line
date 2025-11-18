use crate::prelude::*;

#[search(LoginSession, no_include_deleted, auth = "authenticated")]
fn resolver() {
    let f = get_filter(ctx).await?;
    let o = order_by!(LoginSession[UpdatedAtDesc]);
    (Some(f), Some(o))
}

#[count(LoginSession, no_include_deleted, auth = "authenticated")]
fn resolver() {
    let f = get_filter(ctx).await?;
    Some(f)
}

async fn get_filter(ctx: &Context<'_>) -> Res<LoginSessionFilter> {
    let arc = ctx.auth_with_cache().await?;
    let ls = arc.as_ref().as_ref().ok_or(MyErr::Unauthenticated)?;

    let f = filter!(LoginSession {
        id_ne: ls.id.clone(),
        user_id: ls.user_id.clone(),
    });
    Ok(f)
}
