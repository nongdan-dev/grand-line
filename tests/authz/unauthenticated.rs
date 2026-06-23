// Tests that authz-protected resolvers reject requests with no auth token.

#[path = "./prelude.rs"]
mod prelude;
use prelude::*;

// A request with no Authorization header to an org-realm resolver returns Unauthorized.
#[tokio::test]
async fn no_token_org_realm() -> Res<()> {
    let d = prepare_wildcard().await?;

    // Provide org header but no auth token.
    let mut h = d.h;
    h.append(H_ORG_ID, h_str(&d.org_id1));
    // Intentionally omit H_AUTHORIZATION.
    let s = d.s.data(h).finish();

    let q = "
    query test {
        orgPrimitive
    }
    ";
    exec_assert_err(&s, q, None, &AuthErr::Unauthenticated).await;

    d.tmp.drop().await
}

// A request with no Authorization header to a system-realm resolver returns Unauthenticated.
#[tokio::test]
async fn no_token_system_realm() -> Res<()> {
    let d = prepare_wildcard().await?;

    let s = d.s.data(d.h).finish();

    let q = "
    query test {
        systemPrimitive
    }
    ";
    exec_assert_err(&s, q, None, &AuthErr::Unauthenticated).await;

    d.tmp.drop().await
}
