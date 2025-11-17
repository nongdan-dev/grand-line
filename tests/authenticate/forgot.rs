#[path = "./prelude.rs"]
mod prelude;
use prelude::*;

#[tokio::test]
async fn t() -> Res<()> {
    let d = prepare().await?;
    let s = d.s.data(d.h).finish();

    let q = r#"
    mutation test($data: Forgot!) {
        forgot(data: $data) {
            secret
        }
    }
    "#;
    let v = value!({
        "data": {
            "email": "olivia@example.com",
        },
    });
    exec_assert_ok(&s, q, Some(&v)).await;

    let t = AuthOtp::find().one_or_404(&d.tmp.db).await?;
    let q = r#"
    mutation test($data: AuthOtpResolve!, $password: String!) {
        forgotResolve(data: $data, password: $password) {
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
        "password": "Str0ngP@ssw0rd?",
    });
    let expected = value!({
        "forgotResolve": {
            "inner": {
                "userId": d.user_id,
            },
        },
    });
    exec_assert(&s, q, Some(&v), &expected).await;

    let u = User::find()
        .filter(UserColumn::Email.eq("olivia@example.com"))
        .one_or_404(&d.tmp.db)
        .await?;
    assert!(
        auth_utils::password_eq(&u.password_hashed, "Str0ngP@ssw0rd?"),
        "password should be updated",
    );

    d.tmp.drop().await
}
