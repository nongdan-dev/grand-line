use crate::prelude::*;

#[gql_input]
pub struct Register {
    pub email: Email,
    pub password: String,
}

pub(crate) async fn register_ensure_email_not_exists<U: AuthUser>(
    tx: &DatabaseTransaction,
    email: &str,
) -> Res<()> {
    let exists = U::find()
        .exclude_deleted()
        .filter(U::email_col().eq(email))
        .exists(tx)
        .await?;
    if exists {
        Err(MyErr::RegisterEmailExists)?;
    }
    Ok(())
}

pub(crate) async fn register_impl<U: AuthUser>(
    ctx: &Context<'_>,
    data: Register,
) -> Res<AuthOtpWithSecret> {
    let tx = &*ctx.tx().await?;
    let h = &ctx.auth_config().handlers;

    register_ensure_email_not_exists::<U>(tx, &data.email.0).await?;
    otp_ensure_re_request(ctx, tx, AuthOtpTy::Register, &data.email.0).await?;
    h.password_validate(ctx, &data.password).await?;

    let otp = h.otp(ctx).await?;
    let (otp_salt, otp_hashed) = rand_utils::otp_hash(&otp)?;
    let secret = rand_utils::secret();

    let t = am_create!(AuthOtp {
        ty: AuthOtpTy::Register,
        email: data.email.0,
        secret_hashed: rand_utils::secret_hash(&secret),
        data: AuthOtpDataRegister {
            password_hashed: rand_utils::password_hash(&data.password)?,
        }
        .to_json()?,
        otp_salt,
        otp_hashed,
    })
    .insert(tx)
    .await?;

    h.on_otp_create(ctx, &t, &otp).await?;

    Ok(AuthOtpWithSecret { inner: t, secret })
}
