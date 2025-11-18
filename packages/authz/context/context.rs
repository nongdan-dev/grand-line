use crate::prelude::*;

#[async_trait]
pub trait AuthzContext {
    async fn authz_ensure_in_macro(&self, check: AuthzDirectiveEnsure) -> Res<()>;
    async fn org_header_with_cache(&self) -> Res<Arc<OrgHeaderCache>>;
    async fn org_header_without_cache(&self) -> Res<OrgHeaderCache>;
}

#[async_trait]
impl AuthzContext for Context<'_> {
    async fn authz_ensure_in_macro(&self, check: AuthzDirectiveEnsure) -> Res<()> {
        let tx = &*self.tx().await?;
        let operation = self.field().name();

        let mut q = Policy::find()
            .exclude_deleted()
            .filter(PolicyColumn::Operation.eq(operation));

        let mut sub = Role::find()
            .exclude_deleted()
            .select_only()
            .column(RoleColumn::Id);
        if check.org {
            let org_id = &self.org_header_with_cache().await?.id;
            sub = sub.filter(RoleColumn::OrgId.eq(org_id));
        } else {
            sub = sub.filter(RoleColumn::OrgId.is_null());
        }
        q = q.filter(PolicyColumn::RoleId.in_subquery(sub.into_query()));

        if check.user {
            let user_id = self.auth().await?;
            let mut sub = UserInRole::find()
                .exclude_deleted()
                .select_only()
                .column(UserInRoleColumn::RoleId)
                .filter(UserInRoleColumn::UserId.eq(user_id));
            if check.org {
                let org_id = &self.org_header_with_cache().await?.id;
                sub = sub.filter(UserInRoleColumn::OrgId.eq(org_id))
            } else {
                sub = sub.filter(UserInRoleColumn::OrgId.is_null())
            }
            q = q.filter(PolicyColumn::RoleId.in_subquery(sub.into_query()));
        }

        let policies = q.all(tx).await?;

        for p in policies {
            let inputs = PolicyData::from_json(p.inputs)?;
            let output = PolicyData::from_json(p.output)?;
            if policy_check_inputs(self, &inputs) && policy_check_output(self, &output) {
                return Ok(());
            }
        }

        Err(MyErr::Unauthorized.into())
    }

    async fn org_header_with_cache(&self) -> Res<Arc<OrgHeaderCache>> {
        let arc = self.cache(|| self.org_header_without_cache()).await?;
        Ok(arc)
    }

    async fn org_header_without_cache(&self) -> Res<OrgHeaderCache> {
        let k = self.authz_config().org_id_header_key;
        let v = self.get_header(k)?.trim().to_owned();
        if v.is_empty() {
            Err(MyErr::HeaderOrgId404)?;
        }

        let tx = &*self.tx().await?;
        let org = Org::find()
            .exclude_deleted()
            .filter_by_id(&v)
            .select_only()
            .column(OrgColumn::Id)
            .into_model::<OrgHeaderCache>()
            .one(tx)
            .await?
            .ok_or(MyErr::Unauthorized)?;

        Ok(org)
    }
}

#[derive(FromQueryResult)]
pub struct OrgHeaderCache {
    pub id: String,
}
