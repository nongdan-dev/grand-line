use super::prelude::*;
use chrono::Duration;

#[gql_input]
pub struct AuthOtpResolve {
    pub id: String,
    pub secret: String,
    pub otp: String,
}

#[mutation]
fn authOtpResolve(ty: AuthOtpTy, data: AuthOtpResolve) -> AuthOtpGql {
    ctx.ensure_not_authenticated().await?;

    ensure_auth_otp_resolve(ctx, tx, ty, data)
        .await?
        .into_gql(ctx)
        .await?
}

pub(crate) async fn ensure_auth_otp_resolve(
    ctx: &Context<'_>,
    tx: &DatabaseTransaction,
    ty: AuthOtpTy,
    data: AuthOtpResolve,
) -> Res<AuthOtpSql> {
    let u = AuthOtp::update_many()
        .include_deleted(None)
        .by_id(&data.id)
        .filter(AuthOtpColumn::Ty.eq(ty))
        .filter(AuthOtpColumn::Secret.eq(data.secret))
        .set(AuthOtpActiveModel::defaults_on_update())
        .col_expr(
            AuthOtpColumn::TotalAttempt,
            Expr::col(AuthOtpColumn::TotalAttempt).add(1),
        );

    #[cfg(feature = "postgres")]
    let t = {
        u.exec_with_returning(tx)
            .await?
            .into_iter()
            .next()
            .ok_or(MyErr::OtpResolveInvalid)?
    };
    #[cfg(not(feature = "postgres"))]
    let t = {
        if u.exec(tx).await?.rows_affected == 0 {
            Err(MyErr::OtpResolveInvalid)?;
        }
        AuthOtp::find_by_id(&data.id)
            .include_deleted(None)
            .one(tx)
            .await?
            .ok_or(MyErr::OtpResolveInvalid)?
    };

    let c = &ctx.config().auth;
    if t.otp != data.otp
        || t.total_attempt > c.otp_max_attempt
        || t.created_at + Duration::milliseconds(c.otp_expire_ms) < now()
    {
        Err(MyErr::OtpResolveInvalid)?;
    }

    let t = db_update!(
        tx,
        AuthOtp {
            total_attempt: 0,
            ..t.into_active_model()
        }
    );

    Ok(t)
}

pub(crate) async fn ensure_otp_resend(
    ctx: &Context<'_>,
    tx: &DatabaseTransaction,
    ty: AuthOtpTy,
    email: &str,
) -> Res<()> {
    let t = AuthOtp::find()
        .include_deleted(None)
        .filter(AuthOtpColumn::Ty.eq(ty))
        .filter(AuthOtpColumn::Email.eq(email))
        .one(tx)
        .await?;
    let t = if let Some(t) = t {
        t
    } else {
        return Ok(());
    };

    let c = &ctx.config().auth;
    if t.created_at + Duration::milliseconds(c.otp_resend_ms) > now() {
        Err(MyErr::OtpResendTooSoon)?;
    }

    Ok(())
}
