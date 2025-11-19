#[path = "./prelude.rs"]
mod prelude;
use prelude::*;

#[tokio::test]
async fn ok() -> Res<()> {
    let d = prepare().await?;

    let mut h = d.h;
    h.append("x-org-id", h_str(&d.org_id1));
    let token = d.token1;
    let token = format!("Bearer {token}");
    h.insert("authorization", h_str(&token));

    let s = d.s.data(h).finish();

    let q = r#"
    query test {
        orgPrimitive
    }
    "#;
    exec_assert_ok(&s, q, None).await;

    let q = r#"
    query test {
        org {
            name
        }
    }
    "#;
    let expected = value!({
        "org": {
            "name": "Fringe",
        },
    });
    exec_assert(&s, q, None, &expected).await;

    let q = r#"
    query test {
        systemPrimitive
    }
    "#;
    exec_assert_ok(&s, q, None).await;

    let q = r#"
    query test($orgId: String!) {
        system(orgId: $orgId) {
            name
        }
    }
    "#;
    let v = value!({
        "orgId": d.org_id2,
    });
    let expected = value!({
        "system": {
            "name": "FBI",
        },
    });
    exec_assert(&s, q, Some(&v), &expected).await;

    Ok(())
}

#[tokio::test]
async fn err() -> Res<()> {
    let d = prepare().await?;

    let mut h = d.h;
    h.append("x-org-id", h_str(&d.org_id1));
    let token = d.token2;
    let token = format!("Bearer {token}");
    h.insert("authorization", h_str(&token));

    let s = d.s.data(h).finish();

    let q = r#"
    query test {
        org {
            name
        }
    }
    "#;
    exec_assert_err(&s, q, None, AuthzErr::Unauthorized).await;

    Ok(())
}
