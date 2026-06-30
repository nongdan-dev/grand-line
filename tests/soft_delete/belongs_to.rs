#[path = "./setup.rs"]
mod setup;
use setup::*;

#[tokio::test]
async fn t() -> Res<()> {
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
