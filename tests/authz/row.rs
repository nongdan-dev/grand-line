#[path = "./prelude.rs"]
mod prelude;
use prelude::*;

// ---------------------------------------------------------------------------
// Test setup
// ---------------------------------------------------------------------------

struct Setup {
    tmp: TmpDb,
    schema: GraphQLSchema<Query, EmptyMutation, EmptySubscription>,
}

async fn setup(row_script: Option<&str>) -> Res<Setup> {
    setup_inner("tasks", row_script, None).await
}

async fn setup_with_cfg(row_script: Option<&str>, cfg: AuthzConfig) -> Res<Setup> {
    setup_inner("tasks", row_script, Some(cfg)).await
}

async fn setup_inner(col_key: &str, row_script: Option<&str>, cfg: Option<AuthzConfig>) -> Res<Setup> {
    let wc = col_policy_field(col_policy_fields_wildcard_nested());
    let col = col_policy(col_key.to_owned(), wc.clone(), wc);
    let row = row_script
        .map(|s| row_policy("tasks".to_owned(), s.to_owned()))
        .unwrap_or_default();
    let d = prepare_with_policy(col, row).await?;

    // task1: assigned to user1, belongs to org1
    am_create!(Task {
        title: "Alpha task",
        assignee_id: d.user_id1.clone(),
        org_id: d.org_id1.clone(),
    })
    .exec_without_ctx(&d.tmp.db)
    .await?;

    // task2: assigned to user2, belongs to org2
    am_create!(Task {
        title: "Beta task",
        assignee_id: d.user_id2.clone(),
        org_id: d.org_id2.clone(),
    })
    .exec_without_ctx(&d.tmp.db)
    .await?;

    let mut h = d.h;
    h.append(H_ORG_ID, h_str(&d.org_id1));
    h.insert(H_AUTHORIZATION, h_bearer(&d.token1));
    h.insert(H_ROLE_ID, h_str(&d.role_id1));
    let mut b = d.s;
    if let Some(c) = cfg {
        b = b.data(c);
    }
    Ok(Setup {
        schema: b.data(h).finish(),
        tmp: d.tmp,
    })
}

const Q: &str = "
query {
    tasks(orderBy: [TitleAsc]) {
        title
    }
}
";

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

// No row_policy entry for this resolver -> all tasks returned.
#[tokio::test]
async fn no_row_policy_returns_all() -> Res<()> {
    let d = setup(None).await?;
    let expected = value!({
        "tasks": [{ "title": "Alpha task" }, { "title": "Beta task" }]
    });
    exec_assert(&d.schema, Q, None, &expected).await;
    d.tmp.drop().await
}

// execute_script returning Ok(None) -> no filter applied -> all tasks returned.
#[tokio::test]
async fn script_none_returns_all() -> Res<()> {
    struct NoneHandler;
    #[async_trait]
    impl AuthzHandlers for NoneHandler {
        async fn execute_script(&self, _ctx: &Context<'_>, _script: &str) -> Res<Option<JsonValue>> {
            Ok(None)
        }
    }

    let d = setup_with_cfg(
        Some("any"),
        AuthzConfig {
            handlers: Arc::new(NoneHandler),
            ..Default::default()
        },
    )
    .await?;
    let expected = value!({
        "tasks": [{ "title": "Alpha task" }, { "title": "Beta task" }]
    });
    exec_assert(&d.schema, Q, None, &expected).await;
    d.tmp.drop().await
}

// Handler reads ctx.auth() to get the current user, filters tasks by assignee.
#[tokio::test]
async fn script_filters_tasks_by_assignee() -> Res<()> {
    struct AssigneeHandler;
    #[async_trait]
    impl AuthzHandlers for AssigneeHandler {
        async fn execute_script(&self, ctx: &Context<'_>, _script: &str) -> Res<Option<JsonValue>> {
            let user_id = ctx.auth().await?;
            Ok(Some(json!({ "assignee_id": user_id })))
        }
    }

    let d = setup_with_cfg(
        Some("any"),
        AuthzConfig {
            handlers: Arc::new(AssigneeHandler),
            ..Default::default()
        },
    )
    .await?;
    // user1 is logged in, so only user1's task is returned.
    let expected = value!({ "tasks": [{ "title": "Alpha task" }] });
    exec_assert(&d.schema, Q, None, &expected).await;
    d.tmp.drop().await
}

// Handler reads ctx.authz() to get the current org, filters tasks by org.
#[tokio::test]
async fn script_filters_tasks_by_org() -> Res<()> {
    struct OrgHandler;
    #[async_trait]
    impl AuthzHandlers for OrgHandler {
        async fn execute_script(&self, ctx: &Context<'_>, _script: &str) -> Res<Option<JsonValue>> {
            let org_id = ctx.authz().await?;
            Ok(Some(json!({ "org_id": org_id })))
        }
    }

    let d = setup_with_cfg(
        Some("any"),
        AuthzConfig {
            handlers: Arc::new(OrgHandler),
            ..Default::default()
        },
    )
    .await?;
    // org1 is the request context, only task belonging to org1 is returned.
    let expected = value!({ "tasks": [{ "title": "Alpha task" }] });
    exec_assert(&d.schema, Q, None, &expected).await;
    d.tmp.drop().await
}

// Handler reads both user and org from ctx, filters by both assignee and org.
#[tokio::test]
async fn script_filters_tasks_by_assignee_and_org() -> Res<()> {
    struct BothHandler;
    #[async_trait]
    impl AuthzHandlers for BothHandler {
        async fn execute_script(&self, ctx: &Context<'_>, _script: &str) -> Res<Option<JsonValue>> {
            let user_id = ctx.auth().await?;
            let org_id = ctx.authz().await?;
            Ok(Some(json!({ "assignee_id": user_id, "org_id": org_id })))
        }
    }

    let d = setup_with_cfg(
        Some("any"),
        AuthzConfig {
            handlers: Arc::new(BothHandler),
            ..Default::default()
        },
    )
    .await?;
    let expected = value!({ "tasks": [{ "title": "Alpha task" }] });
    exec_assert(&d.schema, Q, None, &expected).await;
    d.tmp.drop().await
}

// The script string stored in row_policy is forwarded verbatim to execute_script.
#[tokio::test]
async fn script_string_forwarded_verbatim() -> Res<()> {
    const EXPECTED_SCRIPT: &str = "filter_by_assignee_v1";

    struct ScriptCheckHandler;
    #[async_trait]
    impl AuthzHandlers for ScriptCheckHandler {
        async fn execute_script(&self, _ctx: &Context<'_>, script: &str) -> Res<Option<JsonValue>> {
            // Return Alpha's filter only if the correct script was received.
            let filter = if script == EXPECTED_SCRIPT {
                json!({ "title": "Alpha task" })
            } else {
                json!({ "title": "Beta task" })
            };
            Ok(Some(filter))
        }
    }

    let d = setup_with_cfg(
        Some(EXPECTED_SCRIPT),
        AuthzConfig {
            handlers: Arc::new(ScriptCheckHandler),
            ..Default::default()
        },
    )
    .await?;
    let expected = value!({ "tasks": [{ "title": "Alpha task" }] });
    exec_assert(&d.schema, Q, None, &expected).await;
    d.tmp.drop().await
}

// An error from execute_script is masked as InternalServer in the GQL response.
#[tokio::test]
async fn script_error_masked_as_internal_server() -> Res<()> {
    struct ErrorHandler;
    #[async_trait]
    impl AuthzHandlers for ErrorHandler {
        async fn execute_script(&self, _ctx: &Context<'_>, _script: &str) -> Res<Option<JsonValue>> {
            Err(AuthzErr::RowScript("evaluation failed".to_owned()).into())
        }
    }

    let d = setup_with_cfg(
        Some("any"),
        AuthzConfig {
            handlers: Arc::new(ErrorHandler),
            ..Default::default()
        },
    )
    .await?;
    exec_assert_err(&d.schema, Q, None, &CoreGraphQLErr::InternalServer).await;
    d.tmp.drop().await
}

// Col policy with wildcard key "*" still applies the row filter correctly.
#[tokio::test]
async fn wildcard_col_key_with_row_filter() -> Res<()> {
    struct AssigneeHandler;
    #[async_trait]
    impl AuthzHandlers for AssigneeHandler {
        async fn execute_script(&self, ctx: &Context<'_>, _script: &str) -> Res<Option<JsonValue>> {
            let user_id = ctx.auth().await?;
            Ok(Some(json!({ "assignee_id": user_id })))
        }
    }

    let d = setup_inner(
        "*",
        Some("any"),
        Some(AuthzConfig {
            handlers: Arc::new(AssigneeHandler),
            ..Default::default()
        }),
    )
    .await?;
    let expected = value!({ "tasks": [{ "title": "Alpha task" }] });
    exec_assert(&d.schema, Q, None, &expected).await;
    d.tmp.drop().await
}
