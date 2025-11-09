use super::prelude::*;

#[gql_input]
pub struct Login {
    pub email: String,
    pub password: String,
}

#[create(LoginSession, resolver_output)]
async fn login() -> LoginSessionGql {
    // TODO: check anonymous not log in yet

    let u = User::find()
        .include_deleted(None)
        .filter(UserColumn::Email.eq(&data.email))
        .one(tx)
        .await?
        .ok_or(MyErr::LoginIncorrect)?;

    // TODO: check if too many incorrect attempts

    if !password_compare(&data.password, &u.password_hashed) {
        Err(MyErr::LoginIncorrect)?;
    }

    // TODO: reset incorrect attempts

    let ls = db_create!(
        tx,
        LoginSession {
            user_id: u.id,
            ip: ctx.get_ip()?,
            ua: ctx.get_ua()?,
        }
    );

    ctx.set_cookie_login_session(&ls)?;

    // TODO: trigger login success event

    ls.into_gql(ctx).await?
}
