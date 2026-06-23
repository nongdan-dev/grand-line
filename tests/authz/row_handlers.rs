use grand_line::prelude::*;

pub struct NoneHandler;
#[async_trait]
impl AuthzHandlers for NoneHandler {
    async fn execute_script(&self, _ctx: &Context<'_>, _script: &str) -> Res<Option<JsonValue>> {
        Ok(None)
    }
}

pub struct AssigneeHandler;
#[async_trait]
impl AuthzHandlers for AssigneeHandler {
    async fn execute_script(&self, ctx: &Context<'_>, _script: &str) -> Res<Option<JsonValue>> {
        let user_id = ctx.auth().await?;
        let f = json!({
            "assignee_id": user_id,
        });
        Ok(Some(f))
    }
}

pub struct OrgHandler;
#[async_trait]
impl AuthzHandlers for OrgHandler {
    async fn execute_script(&self, ctx: &Context<'_>, _script: &str) -> Res<Option<JsonValue>> {
        let org_id = ctx.authz().await?;
        let f = json!({
            "org_id": org_id,
        });
        Ok(Some(f))
    }
}

pub struct BothHandler;
#[async_trait]
impl AuthzHandlers for BothHandler {
    async fn execute_script(&self, ctx: &Context<'_>, _script: &str) -> Res<Option<JsonValue>> {
        let user_id = ctx.auth().await?;
        let org_id = ctx.authz().await?;
        let f = json!({
            "assignee_id": user_id,
            "org_id": org_id,
        });
        Ok(Some(f))
    }
}

pub const SCRIPT_ALPHA: &str = "mock alpha script";
pub struct ScriptCheckHandler;
#[async_trait]
impl AuthzHandlers for ScriptCheckHandler {
    async fn execute_script(&self, _ctx: &Context<'_>, script: &str) -> Res<Option<JsonValue>> {
        let f = if script == SCRIPT_ALPHA {
            json!({
                "title": "Alpha task",
            })
        } else {
            json!({
                "title": "Beta task",
            })
        };
        Ok(Some(f))
    }
}

pub struct ErrorHandler;
#[async_trait]
impl AuthzHandlers for ErrorHandler {
    async fn execute_script(&self, _ctx: &Context<'_>, _script: &str) -> Res<Option<JsonValue>> {
        Err(AuthzErr::RowScript("evaluation failed".to_owned()).into())
    }
}
