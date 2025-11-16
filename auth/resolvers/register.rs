use crate::prelude::*;

#[gql_input]
pub struct Register {
    pub email: Email,
    pub password: String,
}

#[create(AuthOtp, resolver_output, auth=unauthenticated)]
async fn register() -> AuthOtpWithSecret {
    ensure_email_not_registered(tx, &data.email.0).await?;
    ensure_otp_re_request(ctx, tx, AuthOtpTy::Register, &data.email.0).await?;

    let h = &ctx.auth_config().handlers;
    h.password_validate(ctx, &data.password).await?;

    let otp = h.otp(ctx).await?;
    let (otp_salt, otp_hashed) = auth_utils::otp_hash(&otp)?;
    let t = db_create!(
        tx,
        AuthOtp {
            ty: AuthOtpTy::Register,
            email: data.email.0,
            data: AuthOtpDataRegister {
                password_hashed: auth_utils::password_hash(&data.password)?,
            }
            .to_json()?,
            otp_salt,
            otp_hashed,
        }
    );

    h.on_otp_create(ctx, &t, &otp).await?;

    AuthOtpWithSecret { inner: t }
}

pub(crate) async fn ensure_email_not_registered(tx: &DatabaseTransaction, email: &str) -> Res<()> {
    let exists = User::find()
        .include_deleted(None)
        .filter(UserColumn::Email.eq(email))
        .exists(tx)
        .await?;
    if exists {
        Err(MyErr::RegisterEmailExists)?;
    }
    Ok(())
}
