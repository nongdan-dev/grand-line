#[path = "./prelude.rs"]
mod prelude;
use prelude::*;

#[tokio::test]
async fn t() -> Res<()> {
    let d = prepare().await?;

    let mut h = d.h;
    h.insert("Authorization", h_str(&f!("Bearer {}", d.token)));
    let s = d.s.data(h).finish();

    let q = r#"
    query test {
        loginSessionCurrent {
            user {
                email
            }
        }
    }
    "#;
    let expected = value!({
        "loginSessionCurrent": {
            "user": {
                "email": "olivia@example.com",
            },
        },
    });
    exec_assert(&s, q, None, &expected).await;

    d.tmp.drop().await
}
