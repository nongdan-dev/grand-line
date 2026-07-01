#[path = "./setup.rs"]
mod setup;
use setup::*;

#[tokio::test]
async fn login_session_current_returns_authenticated_user() -> Res<()> {
    let d = setup().await?;

    let mut h = d.h;
    h.insert(H_AUTHORIZATION, h_bearer(&d.token));

    let s = d.s.data(h).finish();

    let q = "
    query test {
        loginSessionCurrent {
            userId
        }
    }
    ";
    let expected = value!({
        "loginSessionCurrent": {
            "userId": d.user_id,
        },
    });
    exec_assert(&s, q, None, &expected).await;

    d.tmp.drop().await
}
