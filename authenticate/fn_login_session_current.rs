use super::prelude::*;

#[query]
async fn loginSessionCurrent() -> Option<LoginSessionWithSecret> {
    ctx.authenticate_opt()
        .await?
        .map(|ls| LoginSessionWithSecret { inner: ls })
}
