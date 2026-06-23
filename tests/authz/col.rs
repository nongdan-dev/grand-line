#[path = "./setup.rs"]
mod setup;
use setup::*;

#[tokio::test]
async fn ok() -> Res<()> {
    let d = setup_with_col_policy(col_policy_with_children("org", "name")).await?;

    let mut h = d.h;
    h.append(H_ORG_ID, h_str(&d.org_id1));
    h.insert(H_AUTHORIZATION, h_bearer(&d.token1));
    h.insert(H_ROLE_ID, h_str(&d.role_id1));

    let s = d.s.data(h).finish();

    // name is in the allowed field map -> ok.
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
async fn err() -> Res<()> {
    let d = setup_with_col_policy(col_policy_with_children("org", "name")).await?;

    let mut h = d.h;
    h.append(H_ORG_ID, h_str(&d.org_id1));
    h.insert(H_AUTHORIZATION, h_bearer(&d.token1));
    h.insert(H_ROLE_ID, h_str(&d.role_id1));

    let s = d.s.data(h).finish();

    // id is not in the allowed field map -> unauthorized.
    let q = "
    query test {
        org {
            id
        }
    }
    ";
    exec_assert_err(&s, q, None, &AuthzErr::Unauthorized).await;

    // Selecting both an allowed (name) and a denied (id) field -> unauthorized.
    let q = "
    query test {
        org {
            id
            name
        }
    }
    ";
    exec_assert_err(&s, q, None, &AuthzErr::Unauthorized).await;

    // orgPrimitive is not in the allowed operation map -> unauthorized.
    let q = "
    query test {
        orgPrimitive
    }
    ";
    exec_assert_err(&s, q, None, &AuthzErr::Unauthorized).await;

    d.tmp.drop().await
}
