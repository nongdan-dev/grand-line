#[path = "./prelude.rs"]
mod prelude;
use prelude::*;

#[tokio::test]
async fn t() -> Res<()> {
    let d = prepare().await?;

    let q = r#"
    mutation test($id: ID!) {
        userDelete(id: $id) {
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

    match User::find_by_id(&d.id1).one(&d.tmp.db).await? {
        Some(u) => assert!(
            u.deleted_at != None,
            "it should be soft delete by default, found deleted_at=None",
        ),
        None => assert!(
            false,
            "it should be soft delete by default, found None returned from db",
        ),
    }

    d.tmp.drop().await
}
