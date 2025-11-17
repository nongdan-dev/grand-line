#[path = "./prelude.rs"]
mod prelude;
use prelude::*;

#[tokio::test]
async fn t() -> Res<()> {
    let d = prepare().await?;

    let q = r#"
    query test($id: ID!) {
        userDetail(id: $id, includeDeleted: true) {
            name
        }
    }
    "#;
    let v = value!({
        "id": d.id2,
    });
    let expected = value!({
        "userDetail": {
            "name": "Peter",
        },
    });
    exec_assert(&d.s, q, Some(&v), &expected).await;

    d.tmp.drop().await
}
