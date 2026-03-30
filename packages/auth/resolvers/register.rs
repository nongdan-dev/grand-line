use crate::prelude::*;

#[gql_input]
pub struct Register {
    pub email: Email,
    pub password: String,
}

#[create(AuthOtp, resolver_output, auth(unauthenticated))]
async fn register() -> AuthOtpWithSecret {
    register_ensure_email_not_exists(tx, &data.email.0).await?;
    otp_ensure_re_request(ctx, tx, AuthOtpTy::Register, &data.email.0).await?;

    let h = &ctx.auth_config().handlers;
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

    AuthOtpWithSecret { inner: t, secret }
}

pub(crate) async fn register_ensure_email_not_exists(
    tx: &DatabaseTransaction,
    email: &str,
) -> Res<()> {
    let exists = User::find()
        .exclude_deleted()
        .filter(UserColumn::Email.eq(email))
        .exists(tx)
        .await?;
    if exists {
        Err(MyErr::RegisterEmailExists)?;
    }
    Ok(())
}
