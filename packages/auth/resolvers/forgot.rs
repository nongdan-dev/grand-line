use crate::prelude::*;

#[gql_input]
pub struct Forgot {
    pub email: Email,
}

pub async fn forgot_impl<U: AuthUser>(ctx: &Context<'_>, data: Forgot) -> Res<AuthOtpWithSecret> {
    let tx = &*ctx.tx().await?;
    let h = &ctx.auth_config().handlers;

    auth_otp_ensure_re_request(ctx, tx, AuthOtpTy::Forgot, &data.email.0).await?;

    let u = U::find()
        .exclude_deleted()
        .filter(U::email_col().eq(&data.email.0))
        .one_or_404(tx)
        .await?;

    let otp = h.otp(ctx).await?;
    let secret = rand_utils::secret();
    let (otp_salt, otp_hashed) = rand_utils::otp_hash(&otp)?;

    let t = am_create!(AuthOtp {
        ty: AuthOtpTy::Forgot,
        email: data.email.0,
        secret_hashed: rand_utils::secret_hash(&secret),
        data: AuthOtpDataForgot {
            user_id: u.get_id().clone(),
        }
        .to_json()?,
        otp_salt,
        otp_hashed,
    })
    .exec_without_ctx(tx)
    .await?;

    h.on_otp_create(ctx, &t, &otp).await?;

    Ok(AuthOtpWithSecret {
        inner: t,
        secret,
    })
}
