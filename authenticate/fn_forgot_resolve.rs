use super::prelude::*;

#[gql_input]
pub struct ForgotResolve {
    pub id: String,
    pub otp: String,
    pub secret: String,
    pub password: String,
}

#[create(AuthOtp, resolver_output)]
async fn forgotResolve() -> LoginSessionGql {
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

    let tdata = AuthOtpDataForgot::from_json(t.data)?;

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
