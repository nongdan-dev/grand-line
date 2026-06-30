#[path = "./setup.rs"]
mod setup;
use setup::*;

#[tokio::test]
async fn t() -> Res<()> {
    let d = setup().await?;

    let q = "
    query test {
        userCount(includeDeleted: true)
    }
    ";
    let expected = value!({
        "userCount": 2,
    });
    exec_assert(&d.s, q, None, &expected).await;

    d.tmp.drop().await
}
