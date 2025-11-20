use crate::prelude::*;

#[async_trait]
pub trait AuthzCacheContext {
    async fn authz_with_cache(
        &self,
        check: AuthzDirectiveEnsure,
    ) -> Res<Arc<Option<AuthzCacheItem>>>;
    async fn authz_without_cache(&self, check: AuthzDirectiveEnsure)
    -> Res<Option<AuthzCacheItem>>;
    async fn authz_cache_or_init(
        &self,
    ) -> Res<Arc<Mutex<HashMap<String, Arc<Option<AuthzCacheItem>>>>>>;
    async fn authz_cache_key(&self) -> Res<String>;
}

#[async_trait]
impl AuthzCacheContext for Context<'_> {
    async fn authz_with_cache(
        &self,
        check: AuthzDirectiveEnsure,
    ) -> Res<Arc<Option<AuthzCacheItem>>> {
        let cache_k = self.authz_cache_key().await?;
        let m = self.authz_cache_or_init().await?;
        let mut guard = m.lock().await;
        if let Some(v) = guard.get(&cache_k) {
            return Ok(v.clone());
        }
        let v = self.authz_without_cache(check).await?;
        let v = Arc::new(v);
        guard.insert(cache_k, v.clone());
        Ok(v)
    }

    async fn authz_without_cache(
        &self,
        check: AuthzDirectiveEnsure,
    ) -> Res<Option<AuthzCacheItem>> {
        let mut q = Role::find()
            .exclude_deleted()
            .filter(RoleColumn::Key.eq(&check.key));

        let mut org = None;

        if check.org {
            let o = self.org_unauthorized().await?;
            q = q.filter(RoleColumn::OrgId.eq(&o.id));
            org = Some(o);
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
                let o = self.org_unauthorized().await?;
                sub = sub.filter(UserInRoleColumn::OrgId.eq(&o.id));
                org = Some(o);
            } else {
                sub = sub.filter(UserInRoleColumn::OrgId.is_null());
            }
            q = q.filter(RoleColumn::Id.in_subquery(sub.into_query()));
        }

        let tx = &*self.tx().await?;
        let roles = q.all(tx).await?;
        let operation = self.field().name();

        for role in roles {
            let map = PolicyOperations::from_json(role.operations.clone())?;
            if let Some(p) = map.get("*").or_else(|| map.get(operation))
                && policy_check_inputs(self, &p.inputs)
                && policy_check_output(self, &p.output)
            {
                return Ok(Some(AuthzCacheItem { role, org }));
            }
        }

        Ok(None)
    }

    async fn authz_cache_or_init(
        &self,
    ) -> Res<Arc<Mutex<HashMap<String, Arc<Option<AuthzCacheItem>>>>>> {
        let m = self.cache(async || Ok(Mutex::new(HashMap::new()))).await?;
        Ok(m)
    }

    async fn authz_cache_key(&self) -> Res<String> {
        let operation_ty = self.get_cache::<AuthzCacheOperationTy>().await?;
        let Some(operation_ty) = operation_ty else {
            return Err(MyErr::MissingMacro.into());
        };
        let field = self.field();
        let operation = field.name();
        let alias = field.alias().unwrap_or_default();
        let k = format!("{operation_ty}:{operation}:{alias}");
        Ok(k)
    }
}
