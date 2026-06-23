#[path = "./setup.rs"]
mod setup;
use setup::*;

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
            "password": "123123",
        },
    });
    let expected = value!({
        "login": {
            "inner": {
                "userId": d.user_id,
            },
        },
    });
    exec_assert(&s, q, Some(v), &expected).await;

    d.tmp.drop().await
}
