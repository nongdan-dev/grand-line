use super::prelude::*;

#[gql_input]
pub struct Forgot {
    pub email: Email,
}

#[create(AuthOtp, resolver_output)]
async fn forgot() -> AuthOtpWithSecret {
    ctx.ensure_not_authenticated().await?;

    let h = &ctx.config().auth.handlers;
    ensure_otp_re_request(ctx, tx, AuthOtpTy::Forgot, &data.email.0).await?;

    let u = User::find()
        .include_deleted(None)
        .filter(UserColumn::Email.eq(&data.email.0))
        .one_or_404(tx)
        .await?;

    let t = db_create!(
        tx,
        AuthOtp {
            ty: AuthOtpTy::Forgot,
            email: data.email.0,
            data: AuthOtpDataForgot { user_id: u.id }.to_json()?,
        }
    );

    h.on_otp_create(ctx, &t).await?;

    AuthOtpWithSecret { inner: t }
}
