#[path = "./prelude.rs"]
mod prelude;
use prelude::*;

#[tokio::test]
async fn t() -> Res<()> {
    let d = prepare().await?;
    let s = d.s.data(d.h).finish();

    let q = r#"
    mutation test($data: Forgot) {
        forgot(data: $data) {
            id
        }
    }
    "#;
    let v = value!({
        "data": {
            "email": "olivia@example.com",
        },
    });
    let _ = exec_assert_ok(&s, q, Some(&v)).await;

    let t = AuthTicket::find().one_or_404(&d.tmp.db).await?;
    let q = r#"
    mutation test($data: ForgotResolve) {
        forgotResolve(data: $data) {
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
            "secret": t.secret.clone(),
            "password": "999999",
        },
    });
    let expected = value!({
        "forgotResolve": {
            "user": {
                "email": "olivia@example.com",
            },
        },
    });
    exec_assert(&s, q, Some(&v), &expected).await;

    let u = User::find()
        .filter(UserColumn::Email.eq("olivia@example.com"))
        .one_or_404(&d.tmp.db)
        .await?;
    assert!(
        password_compare("999999", &u.password_hashed),
        "password should be updated"
    );

    d.tmp.drop().await
}
