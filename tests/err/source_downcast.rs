#[path = "../test_utils/mod.rs"]
mod test_utils;
use test_utils::*;

#[grand_line_err]
enum MyErr {
    #[error("test")]
    Test,
}

struct MyQuery;
#[Object]
impl MyQuery {
    async fn my_err(&self) -> Res<i64> {
        Err(MyErr::Test)?;
        Ok(0)
    }
}

fn schema() -> Schema<MyQuery, EmptyMutation, EmptySubscription> {
    Schema::build(MyQuery, EmptyMutation, EmptySubscription).finish()
}

#[tokio::test]
async fn should_be_my_err() {
    let s = schema();

    let r = s.execute("{ myErr }").await;
    assert!(r.errors.len() == 1, "response should have an error");

    let e = &r.errors[0];
    assert!(e.message == "test", "error message should match");

    let box_dyn = e
        .source
        .as_deref()
        .and_then(|e| e.downcast_ref::<GrandLineErr>());
    assert!(box_dyn.is_some(), "downcast to GrandLineErr should be some");
    assert!(box_dyn.unwrap().0.code() == "Test", "code should match");
}
