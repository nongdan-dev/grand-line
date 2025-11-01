use super::prelude::*;

#[gql_input]
pub struct Register {
    pub email: Email,
    pub password: String,
}

#[create(AuthTicket, resolver_output)]
async fn register() -> AuthTicketGql {
    // TODO: check anonymous not log in yet

    check_email_should_not_exist(tx, &data.email.0).await?;

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

    t.into_gql(ctx).await?
}

#[gql_input]
pub struct RegisterResolve {
    pub id: String,
    pub otp: String,
    pub secret: String,
}

#[create(LoginSession, resolver_output)]
async fn registerResolve() -> LoginSessionGql {
    // TODO: check anonymous not log in yet

    let t = AuthTicket::find_by_id(&data.id)
        .one(tx)
        .await?
        .ok_or(MyErr::OtpResolveInvalid)?;

    // TODO: increase otp total attempts, check <= 3

    if t.id != data.id || t.otp != data.otp || t.secret != data.secret {
        Err(MyErr::OtpResolveInvalid)?;
    }

    // TODO: check otp expired

    let tdata = AuthTicketDataRegister::from_json(t.data)?;

    check_email_should_not_exist(tx, &t.email).await?;

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
    ctx._set_cookie_login_session(&ls)?;

    // TODO: trigger register success event

    ls.into_gql(ctx).await?
}

async fn check_email_should_not_exist(tx: &DatabaseTransaction, email: &String) -> Res<()> {
    let email_exists = User::find()
        .filter(UserColumn::Email.eq(email))
        .exists(tx)
        .await?;
    if email_exists {
        err!(RegisterEmailExists)?;
    }
    Ok(())
}
