use super::prelude::*;

#[model(no_deleted_at, no_by_id)]
pub struct LoginSession {
    #[default(random_secret_256bit())]
    #[graphql(skip)]
    pub secret: String,

    pub user_id: String,
    #[belongs_to]
    pub user: User,

    pub ip: String,
    pub ua: String,
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
