#[path = "./setup.rs"]
mod setup;
use setup::*;

#[tokio::test]
async fn t() -> Res<()> {
    let d = setup().await?;

    let q = "
    query test($id: ID!) {
        userDetail(id: $id, includeDeleted: true) {
            name
        }
    }
    ";
    let v = value!({
        "id": d.id2,
    });
    let expected = value!({
        "userDetail": {
            "name": "Peter",
        },
    });
    exec_assert(&d.s, q, Some(v), &expected).await;

    d.tmp.drop().await
}
