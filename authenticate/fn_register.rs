use super::prelude::*;

#[gql_input]
pub struct Register {
    pub email: Email,
    pub password: String,
}

#[create(AuthOtp, resolver_output)]
async fn register() -> AuthOtpWithSecret {
    ctx.ensure_not_authenticated().await?;

    ensure_email_not_registered(tx, &data.email.0).await?;
    ensure_otp_resend(ctx, tx, AuthOtpTy::Register, &data.email.0).await?;

    let h = &ctx.config().auth.handlers;
    h.validate_password(ctx, &data.password).await?;

    let t = db_create!(
        tx,
        AuthOtp {
            ty: AuthOtpTy::Register,
            email: data.email.0,
            data: AuthOtpDataRegister {
                password_hashed: password_hash(&data.password)?,
            }
            .to_json()?,
        }
    );

    h.on_otp_create(ctx, &t).await?;

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
