use crate::prelude::*;

#[gql_input]
pub struct Login {
    pub email: String,
    pub password: String,
}

#[create(LoginSession, resolver_output)]
async fn login() -> LoginSessionGql {
    // TODO: check anonymous not log in yet

    let u = User::find()
        .filter(UserColumn::Email.eq(data.email))
        .one(tx)
        .await?
        .ok_or(MyErr::LoginIncorrect)?;

    // TODO: check if too many incorrect attempts

    if !password_compare(&data.password, &u.password_hashed)? {
        err!(LoginIncorrect)?;
    }

    // TODO: reset incorrect attempts

    let s = am_create!(LoginSession {
        user_id: u.id,
        secret: secret_256bit(),
    })
    .insert(tx)
    .await?;

    ctx.set_cookie_login_session(&s)?;

    // TODO: trigger login success event

    s.into_gql(ctx).await?
}
