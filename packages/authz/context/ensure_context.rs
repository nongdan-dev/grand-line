use crate::prelude::*;

#[async_trait]
pub trait AuthzEnsureContext {
    async fn authz_ensure_in_macro(&self, check: AuthzDirectiveEnsure) -> Res<()>;
}

#[async_trait]
impl AuthzEnsureContext for Context<'_> {
    async fn authz_ensure_in_macro(&self, check: AuthzDirectiveEnsure) -> Res<()> {
        // authz_with_cache -> authz_cache_key() stores the root key in AuthzCachedKey
        // on first call so nested resolvers return the same HashMap entry.
        let v = self.authz_with_cache(check).await?;
        if v.is_none() {
            return Err(MyErr::Unauthorized.into());
        }
        Ok(())
    }
}
