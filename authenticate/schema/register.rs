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

    // TODO: check if this email has been requested register recently

    let t = db_create!(
        tx,
        AuthTicket {
            ty: AuthTicketTy::Register,
            email: data.email.0,
            data: AuthTicketDataRegister {
                password_hashed: password_hash(&data.password)?,
            }
            .to_json()?,
        }
    );

    // TODO: trigger event otp

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

    let tdata = AuthTicketDataRegister::from_json(t.data)?;

    try_email_should_not_exist(tx, &t.email).await?;

    let u = db_create!(
        tx,
        User {
            email: t.email,
            password_hashed: tdata.password_hashed,
        }
    );
    let ls = db_create!(
        tx,
        LoginSession {
            user_id: u.id,
            ip: ctx.get_ip()?,
            ua: ctx.get_ua()?,
        }
    );
    ctx.set_cookie_login_session(&ls)?;

    // TODO: trigger register success event

    ls.into_gql(ctx).await?
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
