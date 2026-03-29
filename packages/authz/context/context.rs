use crate::prelude::*;

#[async_trait]
pub trait AuthzContext {
    async fn authz(&self) -> Res<String>;
    async fn authz_role(&self) -> Res<RoleSql>;
}

#[async_trait]
impl AuthzContext for Context<'_> {
    async fn authz(&self) -> Res<String> {
        let cache_k = self.authz_cache_key().await?;
        let m = self.authz_cache_or_init().await?;
        let guard = m.lock().await;
        let Some(v) = guard.get(&cache_k) else {
            return Err(MyErr::MissingMacro.into());
        };
        let Some(v) = v.as_ref() else {
            return Err(MyErr::Unauthorized.into());
        };
        let Some(o) = &v.org else {
            return Err(MyErr::Unauthorized.into());
        };
        let org_id = o.id.clone();
        Ok(org_id)
    }
    async fn authz_role(&self) -> Res<RoleSql> {
        let cache_k = self.authz_cache_key().await?;
        let m = self.authz_cache_or_init().await?;
        let guard = m.lock().await;
        let Some(v) = guard.get(&cache_k) else {
            return Err(MyErr::MissingMacro.into());
        };
        let Some(v) = v.as_ref() else {
            return Err(MyErr::Unauthorized.into());
        };
        Ok(v.role.clone())
    }
}
