use crate::prelude::*;

#[async_trait]
pub trait AuthzContext {
    async fn authz(&self) -> Res<String>;
    async fn authz_ensure_in_macro(&self, check: AuthzDirectiveEnsure) -> Res<()>;
    async fn org_header_with_cache(&self) -> Res<Arc<OrgHeaderCache>>;
    async fn org_header_without_cache(&self) -> Res<OrgHeaderCache>;
}

#[async_trait]
impl AuthzContext for Context<'_> {
    async fn authz(&self) -> Res<String> {
        // TODO: get org_id from cache?
        self.authz_ensure_in_macro(AuthzDirectiveEnsure {
            org: true,
            user: true,
            key: None,
        })
        .await?;
        let org_id = self.org_header_with_cache().await?.as_ref().id.to_owned();
        Ok(org_id)
    }

    async fn authz_ensure_in_macro(&self, check: AuthzDirectiveEnsure) -> Res<()> {
        let tx = &*self.tx().await?;
        let operation = self.field().name();

        let mut q = Role::find().exclude_deleted();
        if check.org {
            let org_id = &self.org_header_with_cache().await?.id;
            q = q.filter(RoleColumn::OrgId.eq(org_id));
        } else {
            q = q.filter(RoleColumn::OrgId.is_null());
        }

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
            q = q.filter(RoleColumn::Id.in_subquery(sub.into_query()));
        }

        if let Some(key) = check.key {
            q = q.filter(RoleColumn::Key.eq(key));
        }

        let roles = q.all(tx).await?;

        for r in roles {
            let map = HashMap::<String, OperationPolicy>::from_json(r.operations)?;
            if let Some(p) = map.get("*").or_else(|| map.get(operation))
                && policy_check_inputs(self, &p.inputs)
                && policy_check_output(self, &p.output)
            {
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

        let q = Org::find().exclude_deleted().filter_by_id(&v);

        let tx = &*self.tx().await?;
        let org = OrgHeaderCache::select(q)
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
impl OrgHeaderCache {
    pub fn select(q: Select<Org>) -> Selector<SelectModel<Self>> {
        q.select_only()
            .column(OrgColumn::Id)
            .into_model::<OrgHeaderCache>()
    }
}
