use crate::prelude::*;
use serde::de::DeserializeOwned;

/// Retrieve the row-level filter defined in the current operation's policy.
///
/// Must be called after `authz_ensure_in_macro` has run (i.e. inside a resolver
/// that has the #[authz(...)] attribute). Returns None when the matched policy
/// has no row script, meaning all rows are accessible.
#[async_trait]
pub trait AuthzRowContext {
    async fn authz_row<F>(&self) -> Res<Option<F>>
    where
        F: Sized + Serialize + DeserializeOwned;
}

#[async_trait]
impl AuthzRowContext for Context<'_> {
    async fn authz_row<F>(&self) -> Res<Option<F>>
    where
        F: Sized + Serialize + DeserializeOwned,
    {
        let cache_k = self.authz_cache_key().await?;
        let cache_m = self.authz_cache_or_init().await?;
        let guard = cache_m.lock().await;
        let Some(cached) = guard.get(&cache_k).cloned() else {
            return Err(MyErr::MissingMacro.into());
        };
        drop(guard);

        let Some(item) = cached.as_ref() else {
            return Err(MyErr::Unauthorized.into());
        };

        let operation = self.field().name();
        let map = PolicyOperations::from_json(item.role.operations.clone())?;
        let Some(p) = map.get("*").or_else(|| map.get(operation)) else {
            return Err(MyErr::Unauthorized.into());
        };

        let Some(script) = &p.row else {
            return Ok(None);
        };

        // auth/authz failures become None scope vars so scripts that don't
        // reference current_user / current_org still work in those realms
        let user_id = self.auth().await.ok();
        let org_id = item.org.as_ref().map(|o| o.id.as_str());

        let tx = &*self.tx().await?;
        let cfg = self.authz_config();

        let register = cfg.row_register_fns.clone();
        let json = eval_formula(
            script,
            user_id.as_deref(),
            org_id,
            cfg.row_locale,
            tx,
            &cfg.row_graph,
            &cfg.row_formula_opts,
            move |engine| {
                if let Some(f) = register {
                    f(engine);
                }
            },
        )
        .await?;
        let filter = F::from_json(json).map_err(|e| MyErr::RowScript(e.to_string()))?;
        Ok(Some(filter))
    }
}
