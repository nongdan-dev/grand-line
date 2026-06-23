#[path = "./setup.rs"]
mod setup;
use setup::*;

#[tokio::test]
async fn t() -> Res<()> {
    let d = setup().await?;

    let q = "
    mutation test($id: ID!) {
        userDelete(id: $id, permanent: true) {
            id
        }
    }
    ";
    let v = value!({
        "id": d.id1,
    });
    let expected = value!({
        "userDelete": {
            "id": d.id1,
        },
    });
    exec_assert(&d.s, q, Some(v), &expected).await;

    let count = User::find_by_id(&d.id1).count(&d.tmp.db).await?;
    assert!(count == 0, "it should delete permanently in db, found count={count}");

    d.tmp.drop().await
}
