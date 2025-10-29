use crate::prelude::*;

#[gql_input]
pub struct Register {
    pub email: Email,
    pub password: String,
}

#[create(AuthTicket, resolver_output)]
async fn register() -> String {
    // TODO: check anonymous not log in yet

    try_email_should_not_exist(tx, &data.email.0).await?;

    // TODO: check if this email has been requested to register recently?

    let t = am_create!(AuthTicket {
        ty: AUTH_TICKET_REGISTER.to_string(),
        email: data.email.0,
        password_hashed: password_hash(&data.password)?,
        otp: secret_otp_6digits(),
    })
    .insert(tx)
    .await?;

    // TODO: send email otp

    t.id
}

#[gql_input]
pub struct RegisterResolve {
    pub id: String,
    pub otp: String,
}

#[create(LoginSession, resolver_output)]
async fn registerResolve() -> LoginSessionGql {
    // TODO: check anonymous not log in yet

    let t = AuthTicket::find_by_id(&data.id)
        .one(tx)
        .await?
        .ok_or(MyErr::OtpResolveInvalid)?;

    // TODO: increase otp total attempts, check <= 3

    if t.id != data.id || t.otp != data.otp {
        Err(MyErr::OtpResolveInvalid)?;
    }

    // TODO: check otp expired

    try_email_should_not_exist(tx, &t.email).await?;

    let u = am_create!(User {
        email: t.email,
        password_hashed: t.password_hashed,
    })
    .insert(tx)
    .await?;

    let s = am_create!(LoginSession {
        user_id: u.id,
        secret: secret_256bit(),
    })
    .insert(tx)
    .await?;

    ctx.set_cookie_login_session(&s)?;

    // TODO: trigger register success event

    s.into_gql(ctx).await?
}

async fn try_email_should_not_exist(tx: &DatabaseTransaction, email: &String) -> Res<()> {
    let email_exists = User::find()
        .filter(UserColumn::Email.eq(email))
        .exists(tx)
        .await?;
    if email_exists {
        err!(RegisterEmailExists)?;
    }
    Ok(())
}
