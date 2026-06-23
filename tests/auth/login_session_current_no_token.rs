#[path = "./setup.rs"]
mod setup;
use setup::*;

// loginSessionCurrent returns null when no auth token is present.
#[tokio::test]
async fn t() -> Res<()> {
    let d = setup().await?;
    // No Authorization header injected - schema uses default headers without a token.
    let s = d.s.data(d.h).finish();

    let q = "
    query test {
        loginSessionCurrent {
            userId
        }
    }
    ";
    let expected = value!({
        "loginSessionCurrent": null,
    });
    exec_assert(&s, q, None, &expected).await;

    d.tmp.drop().await
}
