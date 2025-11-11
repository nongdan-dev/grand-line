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
                userId
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
    let _ = exec_assert_ok(&s, q, Some(&v)).await;

    let u = User::find()
        .filter(UserColumn::Email.eq("peter@example.com"))
        .one_or_404(&d.tmp.db)
        .await?;
    assert!(
        password_compare("Str0ngP@ssw0rd?", &u.password_hashed),
        "password should be matched"
    );

    d.tmp.drop().await
}
