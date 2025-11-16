use crate::prelude::*;

#[mutation(auth=unauthenticated)]
async fn forgot_resolve(data: AuthOtpResolve, password: String) -> LoginSessionWithSecret {
    let h = &ctx.auth_config().handlers;
    h.password_validate(ctx, &password).await?;
    let lsd = ensure_login_session_data(ctx)?;
    let t = ensure_otp_resolve(ctx, tx, AuthOtpTy::Forgot, data).await?;

    let d = AuthOtpDataForgot::from_json(t.data)?;

    let u = db_update!(
        tx,
        User {
            id: d.user_id,
            password_hashed: auth_utils::password_hash(&password)?,
        }
    );

    let ls = create_login_session(ctx, tx, &u.id, &lsd).await?;
    AuthOtp::delete_by_id(t.id).exec(tx).await?;

    h.on_forgot_resolve(ctx, &u).await?;

    LoginSessionWithSecret { inner: ls }
}
