#[path = "./prelude.rs"]
mod prelude;
use prelude::*;

// Logout succeeds when the user is authenticated and deletes the session.
#[tokio::test]
async fn t() -> Res<()> {
    let d = prepare().await?;

    let mut h = d.h;
    h.insert(H_AUTHORIZATION, h_bearer(&d.token));
    let s = d.s.data(h).finish();

    let q = "
    mutation test {
        logout {
            id
        }
    }
    ";
    let r = exec_assert_ok(&s, q, None).await;
    let r = r.data.to_json()?;

    let id = r.pointer("/logout/id").unwrap_or_default().as_str().unwrap_or_default();
    assert!(!id.is_empty(), "logout should return the session id");

    d.tmp.drop().await
}

// Logout without a token returns Unauthenticated.
#[tokio::test]
async fn unauthenticated() -> Res<()> {
    let d = prepare().await?;
    let s = d.s.data(d.h).finish();

    let q = "
    mutation test {
        logout {
            userId
        }
    }
    ";
    exec_assert_err(&s, q, None, &AuthErr::Unauthenticated).await;

    d.tmp.drop().await
}
