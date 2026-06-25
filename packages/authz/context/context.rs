use crate::prelude::*;

#[async_trait]
pub trait AuthzContext<'a>
where
    Self: AuthzCacheContext<'a>,
{
    async fn authz(&self) -> Res<String> {
        let k = self.authz_cache_key().await?;
        let m = self.authz_cache_or_init().await?;
        let guard = m.0.lock().await;
        let org_id = guard
            .get(&k)
            .ok_or(MyErr::MissingMacro)?
            .as_ref()
            .as_ref()
            .ok_or_else(|| self.authz_err().clone())?
            .org
            .as_ref()
            .ok_or_else(|| self.authz_err().clone())?
            .id
            .clone();
        drop(guard);
        Ok(org_id)
    }

    async fn authz_role(&self) -> Res<RoleSql> {
        let k = self.authz_cache_key().await?;
        let m = self.authz_cache_or_init().await?;
        let guard = m.0.lock().await;
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
impl<'a> AuthzContext<'a> for Context<'a> {
}
