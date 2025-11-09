use super::prelude::*;

#[gql_input]
pub struct RegisterResolve {
    pub id: String,
    pub otp: String,
    pub secret: String,
}

#[create(LoginSession, resolver_output)]
async fn registerResolve() -> LoginSessionGql {
    // TODO: check anonymous not log in yet

    let t = AuthOtp::find_by_id(&data.id)
        .include_deleted(None)
        .one(tx)
        .await?
        .ok_or(MyErr::OtpResolveInvalid)?;

    // TODO: increase otp total attempts, check <= 3

    if t.id != data.id || t.otp != data.otp || t.secret != data.secret {
        Err(MyErr::OtpResolveInvalid)?;
    }

    // TODO: check otp expired

    let tdata = AuthOtpDataRegister::from_json(t.data)?;

    ensure_email_not_registered(tx, &t.email).await?;

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
