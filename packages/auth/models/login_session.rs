use crate::prelude::*;

#[model(no_deleted_at, no_by_id)]
pub struct LoginSession {
    pub user_id: String,

    #[default(rand_utils::secret())]
    #[graphql(skip)]
    pub secret: String,

    pub ip: String,
    pub ua: JsonValue,
}

/// To only expose secret in some operations, not the others.
pub struct LoginSessionWithSecret {
    pub inner: LoginSessionSql,
}
#[Object]
impl LoginSessionWithSecret {
    pub async fn secret(&self) -> String {
        self.inner.secret.clone()
    }
    pub async fn inner(&self, ctx: &Context<'_>) -> Res<LoginSessionGql> {
        let r = self.inner.clone().into_gql(ctx).await?;
        Ok(r)
    }
}
