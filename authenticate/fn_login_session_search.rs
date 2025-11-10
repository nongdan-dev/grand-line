use super::prelude::*;

#[search(LoginSession)]
fn resolver() {
    ctx.ensure_authenticated().await?;

    let ls = ctx.authenticate().await?;
    let f = filter_some!(LoginSession {
        id_ne: ls.id,
        user_id: ls.user_id,
    });
    let o = order_by_some!(LoginSession[UpdatedAtDesc]);
    (f, o)
}
