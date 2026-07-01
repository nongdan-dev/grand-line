#[path = "./setup.rs"]
mod setup;
use setup::*;

#[tokio::test]
async fn belongs_to_excludes_soft_deleted_by_default() -> Res<()> {
    let d = setup().await?;

    let q = "
    query test($id: ID!) {
        personDetail(id: $id) {
            user {
                name
            }
        }
    }
    ";
    let v = value!({
        "id": d.pid2,
    });
    let expected = value!({
        "personDetail": {
            "user": null,
        },
    });
    exec_assert(&d.s, q, Some(v), &expected).await;

    d.tmp.drop().await
}
