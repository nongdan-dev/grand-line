#[path = "./setup.rs"]
mod setup;
use setup::*;

#[tokio::test]
async fn search_excludes_soft_deleted_by_default() -> Res<()> {
    let d = setup().await?;

    let q = "
    query test {
        userSearch {
            name
        }
    }
    ";
    let expected = value!({
        "userSearch": [{
            "name": "Olivia",
        }],
    });
    exec_assert(&d.s, q, None, &expected).await;

    d.tmp.drop().await
}
