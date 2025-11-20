use crate::prelude::*;

#[async_trait]
pub trait AuthzContext {
    async fn authz(&self) -> Res<String>;
}

#[async_trait]
impl AuthzContext for Context<'_> {
    async fn authz(&self) -> Res<String> {
        // TODO: get org_id from cache?
        self.authz_ensure_in_macro(AuthzDirectiveEnsure {
            org: true,
            user: true,
            key: "admin".to_owned(),
        })
        .await?;
        let org_id = self.org_unauthorized().await?.as_ref().id.to_owned();
        Ok(org_id)
    }
}
