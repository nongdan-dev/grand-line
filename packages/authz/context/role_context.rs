use crate::prelude::*;

#[async_trait]
pub trait AuthzRoleContext<'a>
where
    Self: AuthzCacheContext<'a>,
{
    async fn authz_role(&self) -> Res<RoleSql> {
        let k = self.authz_cache_key().await?;
        let m = self.authz_cache_or_init().await?;
        let guard = m.lock().await;
        let v = guard
            .get(&k)
            .ok_or(MyErr::MissingMacro)?
            .as_ref()
            .as_ref()
            .ok_or_else(|| self.authz_err().clone())?
            .role
            .clone();
        drop(guard);
        Ok(v)
    }
}

#[async_trait]
impl<'a> AuthzRoleContext<'a> for Context<'a> {
}
