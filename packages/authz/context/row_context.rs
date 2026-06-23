use crate::prelude::*;
use serde::de::DeserializeOwned;

// Build "users.posts" style path from the current resolver position,
// skipping list indices so "users.0.posts" becomes "users.posts".
fn authz_field_path(ctx: &Context<'_>) -> String {
    let Some(node) = ctx.path_node.as_ref() else {
        return ctx.field().name().to_string();
    };
    node.to_string_vec()
        .into_iter()
        .filter(|s| s.parse::<usize>().is_err())
        .collect::<Vec<_>>()
        .join(".")
}

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
        let r = self.authz_role().await?;
        let k = authz_field_path(self);

        let Some(script) = r.row_policy.get(k) else {
            return Ok(None);
        };
        if script.is_null() {
            return Ok(None);
        }

        let Some(script) = script.as_str() else {
            let e = "script is not a string in db";
            return Err(MyErr::RowScript(e.to_string()).into());
        };

        let h = &self.authz_config().handlers;
        let Some(json) = h.execute_script(self, script).await? else {
            return Ok(None);
        };

        let f = F::from_json(json).map_err(|e| MyErr::RowScript(e.to_string()))?;
        Ok(Some(f))
    }
}
