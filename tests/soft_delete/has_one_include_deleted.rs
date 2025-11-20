#[path = "./prelude.rs"]
mod prelude;
use prelude::*;

#[tokio::test]
async fn t() -> Res<()> {
    let d = prepare().await?;

    let q = r#"
    query test($id: ID!) {
        userDetail(id: $id) {
            person(includeDeleted: true) {
                gender
            }
        }
    }
    "#;
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
