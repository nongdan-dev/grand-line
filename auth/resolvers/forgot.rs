use crate::prelude::*;

#[gql_input]
pub struct Forgot {
    pub email: Email,
}

#[create(AuthOtp, resolver_output, auth=unauthenticated)]
async fn forgot() -> AuthOtpWithSecret {
    let h = &ctx.auth_config().handlers;
    ensure_otp_re_request(ctx, tx, AuthOtpTy::Forgot, &data.email.0).await?;

    let u = User::find()
        .include_deleted(None)
        .filter(UserColumn::Email.eq(&data.email.0))
        .one_or_404(tx)
        .await?;
    let otp = h.otp(ctx).await?;
    let (otp_salt, otp_hashed) = auth_utils::otp_hash(&otp)?;
    let t = db_create!(
        tx,
        AuthOtp {
            ty: AuthOtpTy::Forgot,
            email: data.email.0,
            data: AuthOtpDataForgot { user_id: u.id }.to_json()?,
            otp_salt,
            otp_hashed,
        },
    );

    h.on_otp_create(ctx, &t, &otp).await?;

    AuthOtpWithSecret { inner: t }
}
