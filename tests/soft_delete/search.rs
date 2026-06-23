#[path = "./setup.rs"]
mod setup;
use setup::*;

#[tokio::test]
async fn t() -> Res<()> {
    let d = setup().await?;

    let q = "
    query test {
        userSearch {
            name
        }
    }
    ";
    let expected = value!({
        "userSearch": [{
            "name": "Olivia",
        }],
    });
    exec_assert(&d.s, q, None, &expected).await;

    d.tmp.drop().await
}
