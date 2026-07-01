#[path = "./setup.rs"]
mod setup;
use setup::*;

#[tokio::test]
async fn search_include_deleted_returns_all() -> Res<()> {
    let d = setup().await?;

    let q = "
    query test {
        userSearch(orderBy: [NameAsc], includeDeleted: true) {
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
