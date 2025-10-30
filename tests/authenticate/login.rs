#[path = "./prelude.rs"]
mod prelude;
use prelude::*;

#[tokio::test]
async fn t() -> Res<()> {
    let d = prepare().await?;
    let s = d.s.data(d.h).finish();

    let q = r#"
    mutation test($data: Login) {
        login(data: $data) {
            user {
                email
            }
        }
    }
    "#;
    let v = value!({
        "data": {
            "email": "olivia@example.com",
            "password": "123123",
        },
    });
    let expected = value!({
        "login": {
            "user": {
                "email": "olivia@example.com",
            },
        },
    });
    exec_assert(&s, q, Some(&v), &expected).await;

    d.tmp.drop().await
}
