use crate::prelude::*;

#[mutation(auth=unauthenticated)]
async fn register_resolve(data: AuthOtpResolve) -> LoginSessionWithSecret {
    let h = &ctx.auth_config().handlers;
    let lsd = ensure_login_session_data(ctx)?;
    let t = ensure_otp_resolve(ctx, tx, AuthOtpTy::Register, data).await?;
    let d = AuthOtpDataRegister::from_json(t.data)?;

    ensure_email_not_registered(tx, &t.email).await?;

    let u = db_create!(
        tx,
        User {
            email: t.email,
            password_hashed: d.password_hashed,
        },
    );

    let ls = create_login_session(ctx, tx, &u.id, &lsd).await?;
    AuthOtp::delete_by_id(t.id).exec(tx).await?;

    h.on_register_resolve(ctx, &u).await?;

    LoginSessionWithSecret { inner: ls }
}
