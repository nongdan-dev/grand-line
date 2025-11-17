use crate::prelude::*;
use chrono::Duration;

#[gql_input]
pub struct AuthOtpResolve {
    pub id: String,
    pub secret: String,
    pub otp: String,
}

#[mutation(auth = 0)]
fn auth_otp_resolve(ty: AuthOtpTy, data: AuthOtpResolve) -> AuthOtpGql {
    otp_ensure_resolve(ctx, tx, ty, data)
        .await?
        .into_gql(ctx)
        .await?
}

pub(crate) async fn otp_ensure_resolve(
    ctx: &Context<'_>,
    tx: &DatabaseTransaction,
    ty: AuthOtpTy,
    data: AuthOtpResolve,
) -> Res<AuthOtpSql> {
    let u = AuthOtp::update_many()
        .filter_by_id(&data.id)
        .filter(AuthOtpColumn::Ty.eq(ty))
        .set(AuthOtpActiveModel::defaults_on_update())
        .col_expr(
            AuthOtpColumn::TotalAttempt,
            Expr::col(AuthOtpColumn::TotalAttempt).add(1),
        );

    #[cfg(feature = "postgres")]
    let t = {
        u.exec_with_returning(tx)
            .await?
            .first()
            .ok_or(MyErr::OtpResolveInvalid)?
            .to_owned()
    };
    #[cfg(not(feature = "postgres"))]
    let t = {
        if u.exec(tx).await?.rows_affected == 0 {
            Err(MyErr::OtpResolveInvalid)?;
        }
        AuthOtp::find_by_id(&data.id)
            .one(tx)
            .await?
            .ok_or(MyErr::OtpResolveInvalid)?
    };

    let c = &ctx.auth_config();
    if !rand_utils::otp_eq(&t.otp_salt, &t.otp_hashed, &data.otp)?
        || !rand_utils::constant_time_eq(&t.secret, &data.secret)
        || t.total_attempt > c.otp_max_attempt
        || t.created_at + Duration::milliseconds(c.otp_expire_ms) < now()
    {
        Err(MyErr::OtpResolveInvalid)?;
    }

    let t = am_update!(AuthOtp {
        total_attempt: 0,
        ..t.into_active_model()
    })
    .update(tx)
    .await?;

    Ok(t)
}

pub(crate) async fn otp_ensure_re_request(
    ctx: &Context<'_>,
    tx: &DatabaseTransaction,
    ty: AuthOtpTy,
    email: &str,
) -> Res<()> {
    let t = AuthOtp::find()
        .filter(AuthOtpColumn::Ty.eq(ty))
        .filter(AuthOtpColumn::Email.eq(email))
        .one(tx)
        .await?;
    let t = if let Some(t) = t {
        t
    } else {
        return Ok(());
    };

    let c = &ctx.auth_config();
    if t.created_at + Duration::milliseconds(c.otp_re_request_ms) > now() {
        Err(MyErr::OtpReRequestTooSoon)?;
    }

    AuthOtp::delete_many()
        .filter(AuthOtpColumn::Ty.eq(ty))
        .filter(AuthOtpColumn::Email.eq(email))
        .exec(tx)
        .await?;

    Ok(())
}
