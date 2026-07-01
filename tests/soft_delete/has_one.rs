#[path = "./setup.rs"]
mod setup;
use setup::*;

#[tokio::test]
async fn has_one_excludes_soft_deleted_by_default() -> Res<()> {
    let d = setup().await?;

    let q = "
    query test($id: ID!) {
        userDetail(id: $id) {
            person {
                gender
            }
        }
    }
    ";
    let v = value!({
        "id": d.id1,
    });
    let expected = value!({
        "userDetail": {
            "person": null,
        },
    });
    exec_assert(&d.s, q, Some(v), &expected).await;

    d.tmp.drop().await
}
