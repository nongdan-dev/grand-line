use crate::prelude::*;

pub struct AuthzCache(pub Mutex<HashMap<String, Arc<Option<AuthzCacheItem>>>>);

#[async_trait]
pub trait AuthzCacheContext<'a>
where
    Self: AuthContext<'a> + AuthzHttpContext<'a> + AuthzColPolicyContext<'a>,
{
    async fn authz_with_cache(&self, check: AuthzDirectiveEnsure) -> Res<Arc<Option<AuthzCacheItem>>> {
        let k = self.authz_cache_key().await?;

        let m = self.authz_cache_or_init().await?;
        let mut guard = m.0.lock().await;
        if let Some(v) = guard.get(&k) {
            let v = Arc::clone(v);
            drop(guard);
            return Ok(v);
        }

        let v = self.authz_without_cache(check).await?;
        let v = Arc::new(v);
        guard.insert(k, Arc::clone(&v));
        drop(guard);

        Ok(v)
    }

    async fn authz_without_cache(&self, check: AuthzDirectiveEnsure) -> Res<Option<AuthzCacheItem>> {
        let k = self.authz_config().role_id_header_key;
        let role_id = self.get_header(k)?.trim().to_owned();
        if role_id.is_empty() {
            return Err(MyErr::HeaderRoleId404.into());
        }

        let mut q = Role::find()
            .exclude_deleted()
            .filter_by_id(&role_id)
            .filter(RoleColumn::Realm.eq(&check.realm));

        let org = if check.org {
            let o = self.org_unchecked().await?;
            q = q.filter(RoleColumn::OrgId.eq(&o.id));
            Some(o)
        } else {
            q = q.filter(RoleColumn::OrgId.is_null());
            None
        };

        if check.user {
            let user_id = self.auth().await?;
            let mut sub = UserInRole::find()
                .exclude_deleted()
                .select_only()
                .column(UserInRoleColumn::RoleId)
                .filter(UserInRoleColumn::UserId.eq(user_id));
            if check.org {
                let o = self.org_unchecked().await?;
                sub = sub.filter(UserInRoleColumn::OrgId.eq(&o.id));
            } else {
                sub = sub.filter(UserInRoleColumn::OrgId.is_null());
            }
            q = q.filter(RoleColumn::Id.in_subquery(sub.into_query()));
        }

        let tx = &*self.tx().await?;
        let role = q.one(tx).await?;

        let Some(role) = role else {
            return Ok(None);
        };

        let operation = self.field_impl().name();
        let map = ColPolicy::from_json(role.col_policy.clone())?;
        if let Some(p) = map.get("*").or_else(|| map.get(operation))
            && self.authz_col_policy_check_inputs(&p.inputs)
            && self.authz_col_policy_check_output(&p.output)
        {
            let c = AuthzCacheItem {
                role,
                org,
            };
            return Ok(Some(c));
        }

        Ok(None)
    }

    async fn authz_cache_or_init(&self) -> Res<Arc<AuthzCache>> {
        self.cache(async || Ok(AuthzCache(Mutex::new(HashMap::new())))).await
    }

    async fn authz_cache_key(&self) -> Res<String> {
        let operation_ty = self.get_cache::<AuthzCacheOperationTy>().await?;
        let Some(operation_ty) = operation_ty else {
            return Err(MyErr::MissingMacro.into());
        };
        // On first call (root resolver), compute and store the key so that nested
        // resolvers (e.g. relations) return the same key instead of their own field name.
        if let Some(cached_key) = self.get_cache::<AuthzCachedKey>().await? {
            return Ok(cached_key.0.clone());
        }
        let field = self.field_impl();
        let operation = field.name();
        let alias = field.alias().unwrap_or_default();
        let k = format!("{operation_ty}:{operation}:{alias}");
        let tobe_moved = k.clone();
        self.cache(async move || Ok(AuthzCachedKey(tobe_moved))).await?;
        Ok(k)
    }
}

#[async_trait]
impl<'a> AuthzCacheContext<'a> for Context<'a> {
}
