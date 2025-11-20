use crate::prelude::*;

#[async_trait]
pub trait AuthzEnsureContext {
    async fn authz_ensure_in_macro(&self, check: AuthzDirectiveEnsure) -> Res<()>;
}

#[async_trait]
impl AuthzEnsureContext for Context<'_> {
    async fn authz_ensure_in_macro(&self, check: AuthzDirectiveEnsure) -> Res<()> {
        let v = self.authz_with_cache(check).await?;
        if v.is_none() {
            Err(MyErr::Unauthorized)?;
        }
        Ok(())
    }
}
