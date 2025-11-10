use super::prelude::*;

#[mutation]
async fn forgotResolve(data: AuthOtpResolve, password: String) -> LoginSessionWithSecret {
    ctx.ensure_not_authenticated().await?;

    let h = &ctx.config().auth.handlers;
    h.validate_password(ctx, &password).await?;
    let lsd = ensure_login_session_data(ctx)?;

    let t = auth_otp_resolve(ctx, tx, data).await?;
    let d = AuthOtpDataForgot::from_json(t.data)?;

    let u = db_update!(
        tx,
        User {
            id: d.user_id,
            password_hashed: password_hash(&password)?,
        }
    );

    let ls = create_login_session(ctx, tx, &u.id, &lsd).await?;

    h.on_forgot_resolve(ctx, &u).await?;

    LoginSessionWithSecret { inner: ls }
}
