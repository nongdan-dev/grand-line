use crate::prelude::*;

#[async_trait]
pub trait RoleUnauthorizedContext {
    async fn role_unchecked(&self) -> Res<Arc<RoleMinimal>>;
    async fn role_unchecked_without_cache(&self) -> Res<RoleMinimal>;
}

#[async_trait]
impl RoleUnauthorizedContext for Context<'_> {
    async fn role_unchecked(&self) -> Res<Arc<RoleMinimal>> {
        let arc = self.cache(|| self.role_unchecked_without_cache()).await?;
        Ok(arc)
    }

    async fn role_unchecked_without_cache(&self) -> Res<RoleMinimal> {
        let k = self.authz_config().role_id_header_key;
        let v = self.get_header(k)?.trim().to_owned();
        if v.is_empty() {
            return Err(MyErr::HeaderRoleId404.into());
        }

        let lookup = self.data_opt::<Arc<dyn AuthzRoleImpl>>().ok_or(MyErr::RoleImplNotFound)?;

        let tx = &*self.tx().await?;
        lookup
            .find_by_id(&v, tx)
            .await?
            .ok_or_else(|| MyErr::Unauthorized.into())
    }
}
