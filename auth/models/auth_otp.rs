use crate::prelude::*;
use chrono::Duration;

#[model(no_updated_at, no_deleted_at, no_by_id)]
pub struct AuthOtp {
    pub email: String,

    #[graphql(skip)]
    pub ty: AuthOtpTy,

    #[default(auth_utils::secret())]
    #[graphql(skip)]
    pub secret: String,

    #[graphql(skip)]
    pub otp_salt: String,
    #[graphql(skip)]
    pub otp_hashed: String,

    #[graphql(skip)]
    pub data: JsonValue,

    #[default(0)]
    #[graphql(skip)]
    pub total_attempt: i64,
    #[resolver(sql_dep=total_attempt)]
    pub remaining_attempt: i64,

    #[resolver(sql_dep=created_at)]
    pub will_expire_at: DateTimeUtc,
    #[resolver(sql_dep=created_at)]
    pub can_re_request_at: DateTimeUtc,
}

#[enunn]
pub enum AuthOtpTy {
    Register,
    Forgot,
}

#[derive(Serialize, Deserialize)]
pub struct AuthOtpDataRegister {
    pub password_hashed: String,
}

#[derive(Serialize, Deserialize)]
pub struct AuthOtpDataForgot {
    pub user_id: String,
}

async fn resolve_remaining_attempt(o: &AuthOtpGql, ctx: &Context<'_>) -> Res<i64> {
    let t = o.total_attempt.ok_or(CoreDbErr::GqlResolverNone)?;
    let m = ctx.auth_config().otp_max_attempt;
    Ok(m - t)
}
async fn resolve_will_expire_at(o: &AuthOtpGql, ctx: &Context<'_>) -> Res<DateTimeUtc> {
    let c = o.created_at.ok_or(CoreDbErr::GqlResolverNone)?;
    let d = Duration::milliseconds(ctx.auth_config().otp_expire_ms);
    Ok(c + d)
}
async fn resolve_can_re_request_at(o: &AuthOtpGql, ctx: &Context<'_>) -> Res<DateTimeUtc> {
    let c = o.created_at.ok_or(CoreDbErr::GqlResolverNone)?;
    let d = Duration::milliseconds(ctx.auth_config().otp_re_request_ms);
    Ok(c + d)
}

/// To only expose secret in some operations, not the others.
pub struct AuthOtpWithSecret {
    pub inner: AuthOtpSql,
}
#[Object]
impl AuthOtpWithSecret {
    pub async fn secret(&self) -> String {
        self.inner.secret.clone()
    }
    pub async fn inner(&self, ctx: &Context<'_>) -> Res<AuthOtpGql> {
        let r = self.inner.clone().into_gql(ctx).await?;
        Ok(r)
    }
}
