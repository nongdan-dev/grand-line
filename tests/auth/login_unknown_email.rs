#[path = "./prelude.rs"]
mod prelude;
use prelude::*;

// Login with an email that does not exist returns LoginIncorrect error
// (same error as wrong password to avoid user enumeration).
#[tokio::test]
async fn t() -> Res<()> {
    let d = prepare().await?;
    let s = d.s.data(d.h).finish();

    let q = "
    mutation test($data: Login!) {
        login(data: $data) {
            inner {
                userId
            }
        }
    }
    ";
    let v = value!({
        "data": {
            "email": "nobody@example.com",
            "password": "123123",
        },
    });
    exec_assert_err(&s, q, Some(v), &AuthErr::LoginIncorrect).await;

    d.tmp.drop().await
}
