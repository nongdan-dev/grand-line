use super::prelude::*;

#[gql_input]
pub struct Forgot {
    pub email: Email,
}

#[create(AuthOtp, resolver_output)]
async fn forgot() -> AuthOtpGql {
    // TODO: check anonymous not log in yet

    let u = User::find()
        .include_deleted(None)
        .filter(UserColumn::Email.eq(&data.email.0))
        .one_or_404(tx)
        .await?;

    // TODO: check if this user id has been requested forgot password recently

    let t = db_create!(
        tx,
        AuthOtp {
            ty: AuthOtpTy::Register,
            email: data.email.0,
            data: AuthOtpDataForgot { user_id: u.id }.to_json()?,
        }
    );

    // TODO: trigger event otp

    t.into_gql(ctx).await?
}
