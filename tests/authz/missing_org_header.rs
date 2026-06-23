// Tests that org-realm resolvers reject requests missing the org-id header.

#[path = "./prelude.rs"]
mod prelude;
use prelude::*;

// An org-realm request with no org-id header returns HeaderOrgId404.
#[tokio::test]
async fn t() -> Res<()> {
    let d = prepare_wildcard().await?;

    // Auth token present but no org id header.
    let mut h = d.h;
    h.insert(H_AUTHORIZATION, h_bearer(&d.token1));
    // Intentionally omit H_ORG_ID.
    let s = d.s.data(h).finish();

    let q = "
    query test {
        orgPrimitive
    }
    ";
    exec_assert_err(&s, q, None, &AuthzErr::HeaderOrgId404).await;

    d.tmp.drop().await
}
