#[path = "./prelude.rs"]
mod prelude;
use prelude::*;

#[tokio::test]
async fn t() -> Res<()> {
    let d = prepare().await?;

    let q = r#"
    query test {
        userSearch(filter: { deletedAt_ne: null }) {
            name
        }
    }
    "#;
    let expected = value!({
        "userSearch": [{
            "name": "Peter",
        }],
    });
    exec_assert(&d.s, q, None, &expected).await;

    let q = r#"
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
    "#;
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
