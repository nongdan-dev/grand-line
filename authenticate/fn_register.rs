use super::prelude::*;

#[gql_input]
pub struct Register {
    pub email: Email,
    pub password: String,
}

#[create(AuthOtp, resolver_output)]
async fn register() -> AuthOtpGql {
    // TODO: check anonymous not log in yet

    ensure_email_not_registered(tx, &data.email.0).await?;

    // TODO: check if this email has been requested register recently

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

    // TODO: trigger event otp

    t.into_gql(ctx).await?
}

pub(crate) async fn ensure_email_not_registered(
    tx: &DatabaseTransaction,
    email: &String,
) -> Res<()> {
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
