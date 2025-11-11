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
            inner {
                userId
            }
        }
    }
    "#;
    let expected = value!({
        "loginSessionCurrent": {
            "inner": {
                "userId": d.user_id.clone(),
            },
        },
    });
    exec_assert(&s, q, None, &expected).await;

    d.tmp.drop().await
}
