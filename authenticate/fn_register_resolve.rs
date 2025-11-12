use super::prelude::*;

#[mutation]
async fn registerResolve(data: AuthOtpResolve) -> LoginSessionWithSecret {
    ctx.ensure_not_authenticated().await?;

    let h = &ctx.config().auth.handlers;
    let lsd = ensure_login_session_data(ctx)?;

    let t = ensure_auth_otp_resolve(ctx, tx, AuthOtpTy::Register, data).await?;
    let d = AuthOtpDataRegister::from_json(t.data)?;

    ensure_email_not_registered(tx, &t.email).await?;

    let u = db_create!(
        tx,
        User {
            email: t.email,
            password_hashed: d.password_hashed,
        }
    );
    let ls = create_login_session(ctx, tx, &u.id, &lsd).await?;

    h.on_register_resolve(ctx, &u).await?;

    LoginSessionWithSecret { inner: ls }
}
