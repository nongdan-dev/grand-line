#[path = "./prelude.rs"]
mod prelude;
use prelude::*;

// Registering with an email that is already in use returns RegisterEmailExists.
#[tokio::test]
async fn t() -> Res<()> {
    let d = prepare().await?;
    let s = d.s.data(d.h).finish();

    let q = "
    mutation test($data: Register!) {
        register(data: $data) {
            secret
        }
    }
    ";
    // olivia@example.com is already seeded in prepare().
    let v = value!({
        "data": {
            "email": "olivia@example.com",
            "password": "Str0ngP@ssw0rd?",
        },
    });
    exec_assert_err(&s, q, Some(v), &AuthErr::RegisterEmailExists).await;

    d.tmp.drop().await
}
