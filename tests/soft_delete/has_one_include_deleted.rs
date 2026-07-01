#[path = "./setup.rs"]
mod setup;
use setup::*;

#[tokio::test]
async fn has_one_include_deleted_returns_soft_deleted() -> Res<()> {
    let d = setup().await?;

    let q = "
    query test($id: ID!) {
        userDetail(id: $id) {
            person(includeDeleted: true) {
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
            "person": {
                "gender": "Female",
            },
        },
    });
    exec_assert(&d.s, q, Some(v), &expected).await;

    d.tmp.drop().await
}
