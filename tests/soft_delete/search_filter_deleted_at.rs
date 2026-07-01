#[path = "./setup.rs"]
mod setup;
use setup::*;

#[tokio::test]
async fn search_filters_by_deleted_at() -> Res<()> {
    let d = setup().await?;

    let q = "
    query test {
        userSearch(
            filter: { deletedAt_ne: null },
        ) {
            name
        }
    }
    ";
    let expected = value!({
        "userSearch": [{
            "name": "Peter",
        }],
    });
    exec_assert(&d.s, q, None, &expected).await;

    let q = "
    query test {
        userSearch(
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
    ";
    let expected = value!({
        "userSearch": [{
            "name": "Olivia",
        }, {
            "name": "Peter",
        }],
    });
    exec_assert(&d.s, q, None, &expected).await;

    d.tmp.drop().await
}
