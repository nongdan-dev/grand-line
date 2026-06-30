#[path = "./setup.rs"]
mod setup;
use setup::*;

#[tokio::test]
async fn t() -> Res<()> {
    let d = setup().await?;

    let q = "
    query test($id: ID!) {
        userDetail(id: $id) {
            name
        }
    }
    ";
    let v = value!({
        "id": d.id2,
    });
    let expected = value!({
        "userDetail": null,
    });
    exec_assert(&d.s, q, Some(v), &expected).await;

    d.tmp.drop().await
}
