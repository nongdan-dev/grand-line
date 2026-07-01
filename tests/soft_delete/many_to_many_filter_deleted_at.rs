#[path = "./setup.rs"]
mod setup;
use setup::*;

#[tokio::test]
async fn many_to_many_filters_by_deleted_at() -> Res<()> {
    let d = setup().await?;

    let q = "
    query test($id: ID!) {
        userDetail(id: $id) {
            orgs(
                filter: { deletedAt_ne: null },
            ) {
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
            }],
        },
    });
    exec_assert(&d.s, q, Some(v), &expected).await;

    let q = "
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
