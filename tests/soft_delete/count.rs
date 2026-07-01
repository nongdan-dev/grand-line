#[path = "./setup.rs"]
mod setup;
use setup::*;

#[tokio::test]
async fn count_excludes_soft_deleted_by_default() -> Res<()> {
    let d = setup().await?;

    let q = "
    query test {
        userCount
    }
    ";
    let expected = value!({
        "userCount": 1,
    });
    exec_assert(&d.s, q, None, &expected).await;

    d.tmp.drop().await
}
