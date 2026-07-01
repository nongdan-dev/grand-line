#[path = "./setup.rs"]
mod setup;
use setup::*;

#[tokio::test]
async fn many_to_many_include_deleted_returns_all() -> Res<()> {
    let d = setup().await?;

    let q = "
    query test($id: ID!) {
        userDetail(id: $id) {
            orgs(orderBy: [NameAsc], includeDeleted: true) {
                name
            }
        }
    }
    ";
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
    exec_assert(&d.s, q, Some(v), &expected).await;

    d.tmp.drop().await
}
