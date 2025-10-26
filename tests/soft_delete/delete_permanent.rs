#[path = "./prelude.rs"]
mod prelude;
use prelude::*;

#[tokio::test]
async fn t() -> Res<()> {
    let d = prepare().await?;

    let q = r#"
    mutation test($id: ID!) {
        userDelete(id: $id, permanent: true) {
            id
        }
    }
    "#;
    let v = value!({
        "id": d.id1.clone(),
    });
    let expected = value!({
        "userDelete": {
            "id": d.id1.clone(),
        },
    });
    exec_assert(&d.s, q, Some(&v), &expected).await;

    match User::find_by_id(&d.id1).count(&d.tmp.db).await? {
        count => assert!(
            count == 0,
            "it should delete permanently in db, found count={}",
            count,
        ),
    }

    d.tmp.drop().await
}
