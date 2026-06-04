use crate::prelude::*;

#[gql_input]
pub struct Login {
    pub email: String,
    pub password: String,
}

pub(crate) struct LoginSessionData {
    pub ip: String,
    pub ua: HashMap<String, String>,
}

pub(crate) fn login_session_data(ctx: &Context<'_>) -> Res<LoginSessionData> {
    Ok(LoginSessionData {
        ip: ctx.get_ip()?,
        ua: ctx.get_ua()?,
    })
}

pub(crate) async fn login_session_create(
    ctx: &Context<'_>,
    tx: &DatabaseTransaction,
    user_id: &str,
    data: &LoginSessionData,
) -> Res<LoginSessionWithSecret> {
    let secret = rand_utils::secret();
    let ls = am_create!(LoginSession {
        user_id: user_id.to_owned(),
        secret_hashed: rand_utils::secret_hash(&secret),
        ip: data.ip.clone(),
        ua: data.ua.to_json()?,
    })
    .insert(tx)
    .await?;

    let lsws = LoginSessionWithSecret {
        inner: ls.clone(),
        secret: secret.clone(),
    };
    ctx.set_cookie_login_session(&lsws)?;
    Ok(lsws)
}

pub(crate) async fn login_impl<U: AuthUser>(
    ctx: &Context<'_>,
    data: Login,
) -> Res<LoginSessionWithSecret> {
    let tx = &*ctx.tx().await?;
    let lsd = login_session_data(ctx)?;

    let u = U::find()
        .exclude_deleted()
        .filter(U::email_col().eq(&data.email))
        .one(tx)
        .await?
        .ok_or(MyErr::LoginIncorrect)?;

    if !rand_utils::password_eq(U::get_password_hashed(&u), &data.password) {
        Err(MyErr::LoginIncorrect)?;
    }

    let ls = login_session_create(ctx, tx, &u.get_id(), &lsd).await?;

    ctx.auth_user_config::<U>()?
        .handlers
        .on_login_resolve(ctx, &u, &ls.inner)
        .await?;

    Ok(ls)
}
