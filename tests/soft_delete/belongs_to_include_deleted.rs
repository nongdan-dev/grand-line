#[path = "./setup.rs"]
mod setup;
use setup::*;

#[tokio::test]
async fn belongs_to_include_deleted_returns_soft_deleted() -> Res<()> {
    let d = setup().await?;

    let q = "
    query test($id: ID!) {
        personDetail(id: $id) {
            user(includeDeleted: true) {
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
            "user": {
                "name": "Peter",
            },
        },
    });
    exec_assert(&d.s, q, Some(v), &expected).await;

    d.tmp.drop().await
}
