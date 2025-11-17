use crate::prelude::*;

#[mutation(auth = 0)]
async fn register_resolve(data: AuthOtpResolve) -> LoginSessionWithSecret {
    let h = &ctx.auth_config().handlers;
    let lsd = login_session_ensure_data(ctx)?;
    let t = otp_ensure_resolve(ctx, tx, AuthOtpTy::Register, data).await?;
    let d = AuthOtpDataRegister::from_json(t.data)?;

    register_ensure_email_not_exists(tx, &t.email).await?;

    let u = am_create!(User {
        email: t.email,
        password_hashed: d.password_hashed,
    })
    .insert(tx)
    .await?;

    let ls = login_session_create(ctx, tx, &u.id, &lsd).await?;
    AuthOtp::delete_by_id(t.id).exec(tx).await?;

    h.on_register_resolve(ctx, &u).await?;

    LoginSessionWithSecret { inner: ls }
}
