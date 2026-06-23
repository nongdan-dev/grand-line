use crate::prelude::*;

#[model(deleted_at = false, by_id = false)]
pub struct LoginSession {
    pub user_id: String,

    #[graphql(skip)]
    pub secret_hashed: String,

    pub ip: String,
    /// User agent in json map of request headers such as user-agent or sec-ch-ua...
    pub ua: JsonValue,
}

/// To only expose secret in some operations, not the others.
pub struct LoginSessionWithSecret {
    pub inner: LoginSessionSql,
    pub secret: String,
}
#[Object]
impl LoginSessionWithSecret {
    pub async fn secret(&self) -> String {
        self.secret.clone()
    }
    pub async fn inner(&self, ctx: &Context<'_>) -> Res<LoginSessionGql> {
        let r = self.inner.clone().into_gql(ctx).await?;
        Ok(r)
    }
}
