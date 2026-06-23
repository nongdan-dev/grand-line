#[path = "./prelude.rs"]
mod prelude;
use prelude::*;

#[tokio::test]
async fn ok_org_realm() -> Res<()> {
    let d = prepare_with_col_wildcard().await?;

    let mut h = d.h;
    h.append(H_ORG_ID, h_str(&d.org_id1));
    h.insert(H_AUTHORIZATION, h_bearer(&d.token1));
    h.insert(H_ROLE_ID, h_str(&d.role_id1));

    let s = d.s.data(h).finish();

    let q = "
    query test {
        orgPrimitive
    }
    ";
    exec_assert_ok(&s, q, None).await;

    let q = "
    query test {
        org {
            name
        }
    }
    ";
    let expected = value!({
        "org": {
            "name": "Fringe",
        },
    });
    exec_assert(&s, q, None, &expected).await;

    d.tmp.drop().await
}

#[tokio::test]
async fn ok_system_realm() -> Res<()> {
    let d = prepare_with_col_wildcard().await?;

    let mut h = d.h;
    h.insert(H_AUTHORIZATION, h_bearer(&d.token1));
    h.insert(H_ROLE_ID, h_str(&d.role_id1_system));

    let s = d.s.data(h).finish();

    let q = "
    query test {
        systemPrimitive
    }
    ";
    exec_assert_ok(&s, q, None).await;

    let q = "
    query test($orgId: String!) {
        system(orgId: $orgId) {
            name
        }
    }
    ";
    let v = value!({
        "orgId": d.org_id2,
    });
    let expected = value!({
        "system": {
            "name": "FBI",
        },
    });
    exec_assert(&s, q, Some(v), &expected).await;

    d.tmp.drop().await
}

#[tokio::test]
async fn err() -> Res<()> {
    let d = prepare_with_col_wildcard().await?;

    // user2's role (role_id2) belongs to org2. Sending org1 header -> role not found
    // (role's OrgId != org1) -> Unauthorized.
    let mut h = d.h;
    h.append(H_ORG_ID, h_str(&d.org_id1));
    h.insert(H_AUTHORIZATION, h_bearer(&d.token2));
    h.insert(H_ROLE_ID, h_str(&d.role_id2));

    let s = d.s.data(h).finish();

    let q = "
    query test {
        org {
            name
        }
    }
    ";
    exec_assert_err(&s, q, None, &AuthzErr::Unauthorized).await;

    d.tmp.drop().await
}
