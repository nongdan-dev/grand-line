#[path = "./prelude.rs"]
mod prelude;
use prelude::*;

#[tokio::test]
async fn t() -> Res<()> {
    let d = prepare().await?;

    let q = r#"
    query test($id: ID!) {
        userDetail(id: $id) {
            orgs(filter: { deletedAt_ne: null }) {
                name
            }
        }
    }
    "#;
    let v = value!({
        "id": d.id1,
    });
    let expected = value!({
        "userDetail": {
            "orgs": [{
                "name": "FBI",
            }],
        },
    });
    exec_assert(&d.s, q, Some(&v), &expected).await;

    let q = r#"
    query test($id: ID!) {
        userDetail(id: $id) {
            orgs(
                filter: {
                    OR: [
                        { deletedAt: null },
                        { deletedAt_ne: null },
                    ],
                },
                orderBy: [NameAsc],
            ) {
                name
            }
        }
    }
    "#;
    let v = value!({
        "id": d.id1,
    });
    let expected = value!({
        "userDetail": {
            "orgs": [{
                "name": "FBI",
            }, {
                "name": "Fringe",
            }],
        },
    });
    exec_assert(&d.s, q, Some(&v), &expected).await;

    d.tmp.drop().await
}
