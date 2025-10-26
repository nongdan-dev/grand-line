#[path = "./prelude.rs"]
mod prelude;
use prelude::*;

#[tokio::test]
async fn t() -> Res<()> {
    let d = prepare().await?;

    let q = r#"
    query test {
        userCount
    }
    "#;
    let expected = value!({
        "userCount": 1,
    });
    exec_assert(&d.s, q, None, &expected).await;

    d.tmp.drop().await
}
