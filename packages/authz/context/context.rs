use crate::prelude::*;

#[async_trait]
pub trait AuthzContext {
    async fn authz(&self) -> Res<String>;
    async fn authz_role(&self) -> Res<RoleSql>;
}

#[async_trait]
impl AuthzContext for Context<'_> {
    async fn authz(&self) -> Res<String> {
        let k = self.authz_cache_key().await?;
        let m = self.authz_cache_or_init().await?;
        let guard = m.lock().await;
        let org_id = guard
            .get(&k)
            .ok_or(MyErr::MissingMacro)?
            .as_ref()
            .as_ref()
            .ok_or(MyErr::Unauthorized)?
            .org
            .as_ref()
            .ok_or(MyErr::Unauthorized)?
            .id
            .clone();
        drop(guard);
        Ok(org_id)
    }
    async fn authz_role(&self) -> Res<RoleSql> {
        let k = self.authz_cache_key().await?;
        let m = self.authz_cache_or_init().await?;
        let guard = m.lock().await;
        let v = guard
            .get(&k)
            .ok_or(MyErr::MissingMacro)?
            .as_ref()
            .as_ref()
            .ok_or(MyErr::Unauthorized)?
            .role
            .clone();
        drop(guard);
        Ok(v)
    }
}
