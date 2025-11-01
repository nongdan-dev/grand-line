#[path = "./prelude.rs"]
mod prelude;
use prelude::*;

#[tokio::test]
async fn t() -> Res<()> {
    let d = prepare().await?;
    let s = d.s.data(d.h).finish();

    let q = r#"
    mutation test($data: Register) {
        register(data: $data)
    }
    "#;
    let v = value!({
        "data": {
            "email": "peter@example.com",
            "password": "123123",
        },
    });
    let _ = exec_assert_ok(&s, q, Some(&v)).await;

    let t = AuthTicket::find().one_or_404(&d.tmp.db).await?;
    let q = r#"
    mutation test($data: RegisterResolve) {
        registerResolve(data: $data) {
            user {
                email
            }
        }
    }
    "#;
    let v = value!({
        "data": {
            "id": t.id.clone(),
            "otp": t.otp.clone(),
        },
    });
    let expected = value!({
        "registerResolve": {
            "user": {
                "email": "peter@example.com",
            },
        },
    });
    exec_assert(&s, q, Some(&v), &expected).await;

    d.tmp.drop().await
}
