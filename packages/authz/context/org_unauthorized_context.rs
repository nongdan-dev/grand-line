use crate::prelude::*;

#[async_trait]
pub trait OrgUnauthorizedContext {
    async fn org_unauthorized(&self) -> Res<Arc<OrgMinimal>>;
    async fn org_unauthorized_without_cache(&self) -> Res<OrgMinimal>;
}

#[async_trait]
impl OrgUnauthorizedContext for Context<'_> {
    async fn org_unauthorized(&self) -> Res<Arc<OrgMinimal>> {
        let arc = self.cache(|| self.org_unauthorized_without_cache()).await?;
        Ok(arc)
    }

    async fn org_unauthorized_without_cache(&self) -> Res<OrgMinimal> {
        let k = self.authz_config().org_id_header_key;
        let v = self.get_header(k)?.trim().to_owned();
        if v.is_empty() {
            Err(MyErr::HeaderOrgId404)?;
        }

        let q = Org::find().exclude_deleted().filter_by_id(&v);

        let tx = &*self.tx().await?;
        let org = OrgMinimal::select(q)
            .one(tx)
            .await?
            .ok_or(MyErr::Unauthorized)?;

        Ok(org)
    }
}
