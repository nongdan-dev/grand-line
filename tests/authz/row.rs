// Integration tests for row-level authorization via authz_row.
//
// Each test creates a schema with a single `rowResult` resolver that calls
// ctx.authz_row::<RowFilter>(). The row_policy on the role is set per test.
// execute_script is mocked via a custom AuthzHandlers implementation.

use grand_line::prelude::*;

#[path = "../_fixtures/user.rs"]
mod user;
use user::*;

#[path = "../_fixtures/org.rs"]
mod org;
use org::*;

// ---------------------------------------------------------------------------
// Filter type deserialized from the execute_script result.
// ---------------------------------------------------------------------------

#[derive(Serialize, Deserialize)]
struct RowFilter {
    assigned_to_id_eq: Option<String>,
    org_id_eq: Option<String>,
}

// ---------------------------------------------------------------------------
// GraphQL output type returned by the test resolver.
// ---------------------------------------------------------------------------

#[derive(SimpleObject)]
struct RowOutput {
    assigned_to_id_eq: Option<String>,
    org_id_eq: Option<String>,
}

// ---------------------------------------------------------------------------
// Test resolver: requires org-realm authz, then returns the row filter.
// authz_row returns None when the role has no row_policy entry for this field.
// ---------------------------------------------------------------------------

#[query(authz(realm = "org"))]
fn row_result() -> Option<RowOutput> {
    ctx.authz().await?;
    let filter: Option<RowFilter> = ctx.authz_row().await?;
    filter.map(|f| RowOutput {
        assigned_to_id_eq: f.assigned_to_id_eq,
        org_id_eq: f.org_id_eq,
    })
}

#[derive(Default, MergedObject)]
struct TestQuery(RowResultQuery);

// ---------------------------------------------------------------------------
// Test setup
// ---------------------------------------------------------------------------

struct Setup {
    tmp: TmpDb,
    schema: GraphQLSchema<TestQuery, EmptyMutation, EmptySubscription>,
    user_id: String,
    org_id: String,
}

async fn prepare(row_script: Option<&str>) -> Res<Setup> {
    prepare_with_op_key("rowResult", row_script).await
}

async fn prepare_with_op_key(col_op_key: &str, row_script: Option<&str>) -> Res<Setup> {
    prepare_inner(col_op_key, row_script, None).await
}

async fn prepare_with_config(row_script: Option<&str>, cfg: AuthzConfig) -> Res<Setup> {
    prepare_inner("rowResult", row_script, Some(cfg)).await
}

async fn prepare_inner(col_op_key: &str, row_script: Option<&str>, cfg: Option<AuthzConfig>) -> Res<Setup> {
    let tmp = tmp_db!(User, LoginSession, Org, Role, UserInRole);
    let mut builder = schema_q::<TestQuery>(&tmp.db).data(authz_org_impl::<Org>());
    if let Some(c) = cfg {
        builder = builder.data(c);
    }
    let h = init_common_headers();

    let u = am_create!(User {
        email: "tester@example.com",
        password_hashed: rand_utils::password_hash("pass123")?,
    })
    .exec_without_ctx(&tmp.db)
    .await?;

    let ua = Context::get_ua_raw(Context::get_headers_raw(&h))?;
    let secret = rand_utils::secret();
    let ls = am_create!(LoginSession {
        user_id: u.id.clone(),
        secret_hashed: rand_utils::secret_hash(&secret),
        ip: "127.0.0.1",
        ua: ua.to_json()?,
    })
    .exec_without_ctx(&tmp.db)
    .await?;
    let token = rand_utils::qs_token(&ls.id, &secret)?;

    let o = am_create!(Org {
        name: "TestOrg",
    })
    .exec_without_ctx(&tmp.db)
    .await?;

    let wc_field = ColPolicyField {
        allow: true,
        children: Some(hashmap! {
            "**".to_owned() => ColPolicyField { allow: true, children: None }
        }),
    };
    let col_p: ColPolicy = hashmap! {
        col_op_key.to_owned() => ColPolicyOperation {
            inputs: wc_field.clone(),
            output: wc_field.clone(),
        }
    };
    // row_policy is a flat map: { "rowResult": "script_string" }
    // The key is the field path resolved by authz_field_path (always "rowResult" here).
    let row_p: JsonValue = match row_script {
        Some(s) => json!({ "rowResult": s }),
        None => json!(null),
    };

    let r = am_create!(Role {
        name: "Tester",
        realm: "org",
        col_policy: col_p.to_json()?,
        row_policy: row_p,
        org_id: Some(o.id.clone()),
    })
    .exec_without_ctx(&tmp.db)
    .await?;
    am_create!(UserInRole {
        user_id: u.id.clone(),
        role_id: r.id.clone(),
        org_id: Some(o.id.clone()),
    })
    .exec_without_ctx(&tmp.db)
    .await?;

    let mut headers = h;
    headers.append(H_ORG_ID, h_str(&o.id));
    headers.insert(H_AUTHORIZATION, h_bearer(&token));
    headers.insert(H_ROLE_ID, h_str(&r.id));
    let schema = builder.data(headers).finish();

    Ok(Setup {
        tmp,
        schema,
        user_id: u.id,
        org_id: o.id,
    })
}

const Q: &str = "
query {
    rowResult {
        assignedToIdEq
        orgIdEq
    }
}
";

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

// When the role has no row_policy entry for this field, authz_row returns None.
#[tokio::test]
async fn no_row_script_returns_null() -> Res<()> {
    let d = prepare(None).await?;
    let expected = value!({ "rowResult": null });
    exec_assert(&d.schema, Q, None, &expected).await;
    d.tmp.drop().await
}

// execute_script returning Ok(None) causes authz_row to return None -> null result.
#[tokio::test]
async fn execute_script_returns_none_gives_null() -> Res<()> {
    struct NoneHandler;
    #[async_trait]
    impl AuthzHandlers for NoneHandler {
        async fn execute_script(&self, _ctx: &Context<'_>, _script: &str) -> Res<Option<JsonValue>> {
            Ok(None)
        }
    }

    let d = prepare_with_config(
        Some("any_script"),
        AuthzConfig {
            handlers: Arc::new(NoneHandler),
            ..Default::default()
        },
    )
    .await?;
    let expected = value!({ "rowResult": null });
    exec_assert(&d.schema, Q, None, &expected).await;
    d.tmp.drop().await
}

// execute_script can read the authenticated user from ctx and inject it into the result.
#[tokio::test]
async fn execute_script_reads_ctx_user() -> Res<()> {
    struct UserHandler;
    #[async_trait]
    impl AuthzHandlers for UserHandler {
        async fn execute_script(&self, ctx: &Context<'_>, _script: &str) -> Res<Option<JsonValue>> {
            let user_id = ctx.auth().await?;
            Ok(Some(json!({ "assigned_to_id_eq": user_id })))
        }
    }

    let d = prepare_with_config(
        Some("any_script"),
        AuthzConfig {
            handlers: Arc::new(UserHandler),
            ..Default::default()
        },
    )
    .await?;
    let expected = value!({
        "rowResult": {
            "assignedToIdEq": d.user_id,
            "orgIdEq": null,
        }
    });
    exec_assert(&d.schema, Q, None, &expected).await;
    d.tmp.drop().await
}

// execute_script can read the current org from ctx and inject it into the result.
#[tokio::test]
async fn execute_script_reads_ctx_org() -> Res<()> {
    struct OrgHandler;
    #[async_trait]
    impl AuthzHandlers for OrgHandler {
        async fn execute_script(&self, ctx: &Context<'_>, _script: &str) -> Res<Option<JsonValue>> {
            let org_id = ctx.authz().await?;
            Ok(Some(json!({ "org_id_eq": org_id })))
        }
    }

    let d = prepare_with_config(
        Some("any_script"),
        AuthzConfig {
            handlers: Arc::new(OrgHandler),
            ..Default::default()
        },
    )
    .await?;
    let expected = value!({
        "rowResult": {
            "assignedToIdEq": null,
            "orgIdEq": d.org_id,
        }
    });
    exec_assert(&d.schema, Q, None, &expected).await;
    d.tmp.drop().await
}

// execute_script can read both user and org from ctx simultaneously.
#[tokio::test]
async fn execute_script_reads_ctx_user_and_org() -> Res<()> {
    struct BothHandler;
    #[async_trait]
    impl AuthzHandlers for BothHandler {
        async fn execute_script(&self, ctx: &Context<'_>, _script: &str) -> Res<Option<JsonValue>> {
            let user_id = ctx.auth().await?;
            let org_id = ctx.authz().await?;
            Ok(Some(json!({
                "assigned_to_id_eq": user_id,
                "org_id_eq": org_id,
            })))
        }
    }

    let d = prepare_with_config(
        Some("any_script"),
        AuthzConfig {
            handlers: Arc::new(BothHandler),
            ..Default::default()
        },
    )
    .await?;
    let expected = value!({
        "rowResult": {
            "assignedToIdEq": d.user_id,
            "orgIdEq": d.org_id,
        }
    });
    exec_assert(&d.schema, Q, None, &expected).await;
    d.tmp.drop().await
}

// The script string stored in row_policy is forwarded verbatim to execute_script.
#[tokio::test]
async fn execute_script_receives_script_string() -> Res<()> {
    const SCRIPT: &str = "my_custom_script_content";

    struct ScriptCaptureHandler;
    #[async_trait]
    impl AuthzHandlers for ScriptCaptureHandler {
        async fn execute_script(&self, _ctx: &Context<'_>, script: &str) -> Res<Option<JsonValue>> {
            let received = script == SCRIPT;
            Ok(Some(json!({ "assigned_to_id_eq": received.to_string() })))
        }
    }

    let d = prepare_with_config(
        Some(SCRIPT),
        AuthzConfig {
            handlers: Arc::new(ScriptCaptureHandler),
            ..Default::default()
        },
    )
    .await?;
    let expected = value!({
        "rowResult": {
            "assignedToIdEq": "true",
            "orgIdEq": null,
        }
    });
    exec_assert(&d.schema, Q, None, &expected).await;
    d.tmp.drop().await
}

// An error returned from execute_script is masked as InternalServer in the GQL response.
#[tokio::test]
async fn execute_script_error_is_internal_server() -> Res<()> {
    struct ErrorHandler;
    #[async_trait]
    impl AuthzHandlers for ErrorHandler {
        async fn execute_script(&self, _ctx: &Context<'_>, _script: &str) -> Res<Option<JsonValue>> {
            Err(AuthzErr::RowScript("script evaluation failed".to_owned()).into())
        }
    }

    let d = prepare_with_config(
        Some("any_script"),
        AuthzConfig {
            handlers: Arc::new(ErrorHandler),
            ..Default::default()
        },
    )
    .await?;
    exec_assert_err(&d.schema, Q, None, &CoreGraphQLErr::InternalServer).await;
    d.tmp.drop().await
}

// Col policy wildcard "*" matches any operation. Row policy is looked up by
// the actual field path independently of the col policy key.
#[tokio::test]
async fn wildcard_col_policy_with_row_policy() -> Res<()> {
    struct UserHandler;
    #[async_trait]
    impl AuthzHandlers for UserHandler {
        async fn execute_script(&self, ctx: &Context<'_>, _script: &str) -> Res<Option<JsonValue>> {
            let user_id = ctx.auth().await?;
            Ok(Some(json!({ "assigned_to_id_eq": user_id })))
        }
    }

    // col_policy uses "*" to match any operation name
    let d = prepare_inner(
        "*",
        Some("any_script"),
        Some(AuthzConfig {
            handlers: Arc::new(UserHandler),
            ..Default::default()
        }),
    )
    .await?;
    let expected = value!({
        "rowResult": {
            "assignedToIdEq": d.user_id,
            "orgIdEq": null,
        }
    });
    exec_assert(&d.schema, Q, None, &expected).await;
    d.tmp.drop().await
}

// execute_script can return any hardcoded value, independent of the script content.
#[tokio::test]
async fn execute_script_returns_hardcoded_value() -> Res<()> {
    struct HardcodedHandler;
    #[async_trait]
    impl AuthzHandlers for HardcodedHandler {
        async fn execute_script(&self, _ctx: &Context<'_>, _script: &str) -> Res<Option<JsonValue>> {
            Ok(Some(json!({ "assigned_to_id_eq": "gold" })))
        }
    }

    let d = prepare_with_config(
        Some("any_script"),
        AuthzConfig {
            handlers: Arc::new(HardcodedHandler),
            ..Default::default()
        },
    )
    .await?;
    let expected = value!({
        "rowResult": {
            "assignedToIdEq": "gold",
            "orgIdEq": null,
        }
    });
    exec_assert(&d.schema, Q, None, &expected).await;
    d.tmp.drop().await
}
