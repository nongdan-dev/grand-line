#[path = "./prelude.rs"]
mod prelude;
use prelude::*;

#[tokio::test]
async fn t() -> Res<()> {
    let d = prepare().await?;

    let mut h = d.h;
    let token = &d.token;
    let token = f!("Bearer {token}");
    h.insert("Authorization", h_str(&token));

    let s = d.s.data(h).finish();

    let q = r#"
    query test {
        loginSessionCurrent {
            userId
        }
    }
    "#;
    let expected = value!({
        "loginSessionCurrent": {
            "userId": d.user_id.clone(),
        },
    });
    exec_assert(&s, q, None, &expected).await;

    d.tmp.drop().await
}
