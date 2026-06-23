#[path = "./setup.rs"]
mod setup;
use setup::*;

// Login with incorrect password returns LoginIncorrect error.
#[tokio::test]
async fn t() -> Res<()> {
    let d = setup().await?;
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
            "email": "olivia@example.com",
            "password": "wrongpassword",
        },
    });
    exec_assert_err(&s, q, Some(v), &AuthErr::LoginIncorrect).await;

    d.tmp.drop().await
}
