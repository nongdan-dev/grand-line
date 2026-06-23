#[path = "./prelude.rs"]
mod prelude;
use prelude::*;

// Submitting a wrong OTP during registration resolve returns OtpResolveInvalid.
#[tokio::test]
async fn t() -> Res<()> {
    let d = prepare().await?;
    let s = d.s.data(d.h).finish();

    // First, start a registration to create an AuthOtp row.
    let q = "
    mutation test($data: Register!) {
        register(data: $data) {
            secret
        }
    }
    ";
    let v = value!({
        "data": {
            "email": "peter@example.com",
            "password": "Str0ngP@ssw0rd?",
        },
    });
    let r = exec_assert_ok(&s, q, Some(v)).await;
    let r = r.data.to_json()?;

    let secret = r
        .pointer("/register/secret")
        .unwrap_or_default()
        .as_str()
        .unwrap_or_default();
    assert!(!secret.is_empty(), "secret should be in response");

    let t = AuthOtp::find().one_or_404(&d.tmp.db).await?;

    // Resolve with the wrong OTP code.
    let q = "
    mutation test($data: AuthOtpResolve!) {
        registerResolve(data: $data) {
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
            "otp": "000000",
        },
    });
    exec_assert_err(&s, q, Some(v), &AuthErr::OtpResolveInvalid).await;

    d.tmp.drop().await
}
