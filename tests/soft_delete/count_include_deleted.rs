#[path = "./setup.rs"]
mod setup;
use setup::*;

#[tokio::test]
async fn count_include_deleted_returns_all() -> Res<()> {
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
