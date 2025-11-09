use super::prelude::*;

#[model]
pub struct AuthOtp {
    pub ty: AuthOtpTy,

    #[graphql(skip)]
    pub email: String,

    #[default(random_otp_6digits())]
    #[graphql(skip)]
    pub otp: String,

    #[default(random_secret_256bit())]
    #[graphql(skip)]
    pub secret: String,

    #[graphql(skip)]
    pub data: JsonValue,

    #[default(0)]
    pub total_attempt: i64,
}

pub struct AuthOtpGqlSecret {
    inner: AuthOtpSql,
}
#[Object]
impl AuthOtpGqlSecret {
    pub async fn secret(&self) -> String {
        self.inner.secret.clone()
    }
    pub async fn data(&self, ctx: &Context<'_>) -> Res<AuthOtpGql> {
        let r = self.inner.clone().into_gql(ctx).await?;
        Ok(r)
    }
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
