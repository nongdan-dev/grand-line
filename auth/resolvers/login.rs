use crate::prelude::*;

#[gql_input]
pub struct Login {
    pub email: String,
    pub password: String,
}

#[create(LoginSession, resolver_output, auth=unauthenticated)]
async fn login() -> LoginSessionWithSecret {
    let h = &ctx.auth_config().handlers;
    let lsd = ensure_login_session_data(ctx)?;

    let u = User::find()
        .include_deleted(None)
        .filter(UserColumn::Email.eq(&data.email))
        .one(tx)
        .await?
        .ok_or(MyErr::LoginIncorrect)?;

    if !auth_utils::password_eq(&u.password_hashed, &data.password) {
        Err(MyErr::LoginIncorrect)?;
    }

    let ls = create_login_session(ctx, tx, &u.id, &lsd).await?;

    h.on_login_resolve(ctx, &u).await?;

    LoginSessionWithSecret { inner: ls }
}

pub(crate) struct EnsureLoginSessionData {
    pub ip: String,
    pub ua: HashMap<String, String>,
}
/// Prepare first to ensure the request context headers are valid before calling other logic.
pub(crate) fn ensure_login_session_data(ctx: &Context<'_>) -> Res<EnsureLoginSessionData> {
    Ok(EnsureLoginSessionData {
        ip: ctx.get_ip()?,
        ua: ctx.get_ua()?,
    })
}

pub(crate) async fn create_login_session(
    ctx: &Context<'_>,
    tx: &DatabaseTransaction,
    user_id: &str,
    data: &EnsureLoginSessionData,
) -> Res<LoginSessionSql> {
    let ls = db_create!(
        tx,
        LoginSession {
            user_id: user_id.to_owned(),
            ip: data.ip.clone(),
            ua: data.ua.clone().to_json()?,
        }
    );
    ctx.set_cookie_login_session(&ls)?;
    Ok(ls)
}
