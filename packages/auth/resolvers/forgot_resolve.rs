use crate::prelude::*;

#[mutation(auth(unauthenticated))]
async fn forgot_resolve(data: AuthOtpResolve, password: String) -> LoginSessionWithSecret {
    let h = &ctx.auth_config().handlers;
    h.password_validate(ctx, &password).await?;
    let lsd = login_session_ensure_data(ctx)?;
    let t = otp_ensure_resolve(ctx, tx, AuthOtpTy::Forgot, data).await?;

    let d = AuthOtpDataForgot::from_json(t.data)?;

    let u = am_update!(User {
        id: d.user_id,
        password_hashed: rand_utils::password_hash(&password)?,
    })
    .update(tx)
    .await?;

    let ls = login_session_create(ctx, tx, &u.id, &lsd).await?;
    AuthOtp::delete_by_id(t.id).exec(tx).await?;

    h.on_forgot_resolve(ctx, &u).await?;

    LoginSessionWithSecret { inner: ls }
}
