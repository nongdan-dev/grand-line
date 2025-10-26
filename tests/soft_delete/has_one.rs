#[path = "./prelude.rs"]
mod prelude;
use prelude::*;

#[tokio::test]
async fn t() -> Res<()> {
    let d = prepare().await?;

    let q = r#"
    query test($id: ID!) {
        userDetail(id: $id) {
            person {
                gender
            }
        }
    }
    "#;
    let v = value!({
        "id": d.id1.clone(),
    });
    let expected = value!({
        "userDetail": {
            "person": null,
        },
    });
    exec_assert(&d.s, q, Some(&v), &expected).await;

    d.tmp.drop().await
}
