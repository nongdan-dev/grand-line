use crate::prelude::*;

#[gql_input]
pub struct Login {
    pub email: String,
    pub password: String,
}

#[create(LoginSession, resolver_output, auth = "unauthenticated")]
async fn login() -> LoginSessionWithSecret {
    let h = &ctx.auth_config().handlers;
    let lsd = login_session_ensure_data(ctx)?;

    let u = User::find()
        .exclude_deleted()
        .filter(UserColumn::Email.eq(&data.email))
        .one(tx)
        .await?
        .ok_or(MyErr::LoginIncorrect)?;

    if !rand_utils::password_eq(&u.password_hashed, &data.password) {
        Err(MyErr::LoginIncorrect)?;
    }

    let ls = login_session_create(ctx, tx, &u.id, &lsd).await?;

    h.on_login_resolve(ctx, &u).await?;

    LoginSessionWithSecret { inner: ls }
}

pub(crate) struct LoginSessionEnsureData {
    pub ip: String,
    pub ua: HashMap<String, String>,
}
/// Prepare first to ensure the request context headers are valid before calling other logic.
pub(crate) fn login_session_ensure_data(ctx: &Context<'_>) -> Res<LoginSessionEnsureData> {
    Ok(LoginSessionEnsureData {
        ip: ctx.get_ip()?,
        ua: ctx.get_ua()?,
    })
}

pub(crate) async fn login_session_create(
    ctx: &Context<'_>,
    tx: &DatabaseTransaction,
    user_id: &str,
    data: &LoginSessionEnsureData,
) -> Res<LoginSessionSql> {
    let ls = am_create!(LoginSession {
        user_id: user_id.to_owned(),
        ip: data.ip.clone(),
        ua: data.ua.to_json()?,
    })
    .insert(tx)
    .await?;
    ctx.set_cookie_login_session(&ls)?;
    Ok(ls)
}
