use crate::prelude::*;

#[async_trait]
pub trait AuthzCacheContext {
    async fn authz_ensure_in_macro(&self, check: AuthzDirectiveEnsure) -> Res<()>;
}

#[async_trait]
impl AuthzCacheContext for Context<'_> {
    async fn authz_ensure_in_macro(&self, check: AuthzDirectiveEnsure) -> Res<()> {
        let tx = &*self.tx().await?;
        let operation = self.field().name();

        let mut q = Role::find()
            .exclude_deleted()
            .filter(RoleColumn::Key.eq(check.key));
        if check.org {
            let org_id = &self.org_unauthorized().await?.id;
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
                let org_id = &self.org_unauthorized().await?.id;
                sub = sub.filter(UserInRoleColumn::OrgId.eq(org_id))
            } else {
                sub = sub.filter(UserInRoleColumn::OrgId.is_null())
            }
            q = q.filter(RoleColumn::Id.in_subquery(sub.into_query()));
        }

        let roles = q.all(tx).await?;

        for r in roles {
            let map = HashMap::<String, PolicyOperation>::from_json(r.operations)?;
            if let Some(p) = map.get("*").or_else(|| map.get(operation))
                && policy_check_inputs(self, &p.inputs)
                && policy_check_output(self, &p.output)
            {
                return Ok(());
            }
        }

        Err(MyErr::Unauthorized.into())
    }
}
