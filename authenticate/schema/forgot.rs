use crate::prelude::*;

#[gql_input]
pub struct Forgot {
    pub email: Email,
}

#[create(AuthTicket, resolver_output)]
async fn forgot() -> String {
    // TODO: check anonymous not log in yet

    let u = User::find()
        .filter(UserColumn::Email.eq(&data.email.0))
        .try_one(tx)
        .await?;

    // TODO: check if this user id has been requested forgot password recently

    let t = db_create!(
        tx,
        AuthTicket {
            ty: AuthTicketTy::Register,
            email: data.email.0,
            data: AuthTicketDataForgot { user_id: u.id }.to_json()?,
        }
    );

    // TODO: trigger event otp

    t.id
}

#[gql_input]
pub struct ForgotResolve {
    pub id: String,
    pub otp: String,
    pub password: String,
}

#[create(AuthTicket, resolver_output)]
async fn forgotResolve() -> LoginSessionGql {
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

    let tdata = AuthTicketDataForgot::from_json(t.data)?;

    let u = db_update!(
        tx,
        User {
            id: tdata.user_id,
            password_hashed: password_hash(&data.password)?,
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

    ls.into_gql(ctx).await?
}
