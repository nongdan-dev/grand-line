#[path = "./prelude.rs"]
mod prelude;
use prelude::*;

#[tokio::test]
async fn t() -> Res<()> {
    let d = prepare().await?;
    let s = d.s.data(d.h).finish();

    let q = r#"
    mutation test($data: Register!) {
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
    exec_assert_ok(&s, q, Some(&v)).await;

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
            "id": t.id,
            "secret": t.secret,
            "otp": "999999",
        },
    });
    exec_assert_ok(&s, q, Some(&v)).await;

    let u = User::find()
        .filter(UserColumn::Email.eq("peter@example.com"))
        .one_or_404(&d.tmp.db)
        .await?;
    assert!(
        rand_utils::password_eq(&u.password_hashed, "Str0ngP@ssw0rd?"),
        "password should be matched",
    );

    d.tmp.drop().await
}
