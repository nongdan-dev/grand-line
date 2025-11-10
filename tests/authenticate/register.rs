#[path = "./prelude.rs"]
mod prelude;
use prelude::*;

#[tokio::test]
async fn t() -> Res<()> {
    let d = prepare().await?;
    let s = d.s.data(d.h).finish();

    let q = r#"
    mutation test($data: Register) {
        register(data: $data) {
            secret
        }
    }
    "#;
    let v = value!({
        "data": {
            "email": "peter@example.com",
            "password": "Str0ngP@ssw0rd?",
        },
    });
    let _ = exec_assert_ok(&s, q, Some(&v)).await;

    let t = AuthOtp::find().one_or_404(&d.tmp.db).await?;
    let q = r#"
    mutation test($data: AuthOtpResolve!) {
        registerResolve(data: $data) {
            inner {
                user {
                    email
                }
            }
        }
    }
    "#;
    let v = value!({
        "data": {
            "id": t.id.clone(),
            "otp": t.otp.clone(),
            "secret": t.secret.clone(),
        },
    });
    let expected = value!({
        "registerResolve": {
            "inner": {
                "user": {
                    "email": "peter@example.com",
                },
            },
        },
    });
    exec_assert(&s, q, Some(&v), &expected).await;

    d.tmp.drop().await
}
