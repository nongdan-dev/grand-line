#[path = "./prelude.rs"]
mod prelude;
use prelude::*;

#[tokio::test]
async fn t() -> Res<()> {
    let d = prepare().await?;
    let s = d.s.data(d.h).finish();

    let q = "
    mutation test($data: Forgot!) {
        forgot(data: $data) {
            secret
        }
    }
    ";
    let v = value!({
        "data": {
            "email": "olivia@example.com",
        },
    });
    let r = exec_assert_ok(&s, q, Some(v)).await;
    let r = r.data.to_json()?;

    let secret = r
        .pointer("/forgot/secret")
        .unwrap_or_default()
        .as_str()
        .unwrap_or_default();
    assert!(!secret.is_empty(), "secret should be in response");

    let t = AuthOtp::find().one_or_404(&d.tmp.db).await?;
    let q = "
    mutation test($data: AuthOtpResolve!, $password: String!) {
        forgotResolve(data: $data, password: $password) {
            inner {
                userId
            }
        }
    }
    ";
    let v = value!({
        "data": {
            "id": t.id,
            "secret": secret,
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
    exec_assert(&s, q, Some(v), &expected).await;

    let u = User::find()
        .filter(UserColumn::Email.eq("olivia@example.com"))
        .one_or_404(&d.tmp.db)
        .await?;

    let password_eq = rand_utils::password_eq(&u.password_hashed, "Str0ngP@ssw0rd?");
    assert!(password_eq, "password should be updated");

    d.tmp.drop().await
}
