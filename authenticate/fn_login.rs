use super::prelude::*;

#[gql_input]
pub struct Login {
    pub email: String,
    pub password: String,
}

#[create(LoginSession, resolver_output)]
async fn login() -> LoginSessionWithSecret {
    ctx.ensure_not_authenticated().await?;

    let h = &ctx.config().auth.handlers;
    let lsd = ensure_login_session_data(ctx)?;

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

    let ls = create_login_session(ctx, tx, &u.id, &lsd).await?;

    h.on_login_resolve(ctx, &u).await?;

    LoginSessionWithSecret { inner: ls }
}

pub(crate) struct EnsureLoginSessionData {
    pub ip: String,
    pub ua: String,
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
            user_id: user_id.to_string(),
            ip: data.ip.to_string(),
            ua: data.ua.to_string(),
        }
    );
    ctx.set_cookie_login_session(&ls)?;
    Ok(ls)
}
