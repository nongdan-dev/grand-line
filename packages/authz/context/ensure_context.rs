use crate::prelude::*;

pub struct AuthzEnsure {
    pub realm: String,
    pub org: bool,
    pub user: bool,
    pub operation: String,
}

#[async_trait]
pub trait AuthzEnsureContext<'a>
where
    Self: AuthzCacheContext<'a>,
{
    async fn authz_ensure_in_macro(&self, check: AuthzEnsure) -> Res<()> {
        let v = self.authz_with_cache(check).await?;
        if v.is_none() {
            return Err(self.authz_err().clone());
        }
        Ok(())
    }
}

#[async_trait]
impl<'a> AuthzEnsureContext<'a> for Context<'a> {
}
