pub use grand_line::prelude::*;

#[grand_line_err]
enum MyErr {
    #[error("test")]
    Test,
}

#[derive(Default)]
struct Query;
#[Object]
impl Query {
    async fn my_err(&self) -> Res<i64> {
        Err(MyErr::Test)?;
        Ok(0)
    }
}

fn schema() -> Schema<Query, EmptyMutation, EmptySubscription> {
    Schema::build(Query, EmptyMutation, EmptySubscription).finish()
}

#[tokio::test]
async fn should_be_my_err() {
    let s = schema();

    let r = s.execute("{ myErr }").await;
    assert!(r.errors.len() == 1, "response should have an error");

    let e = &r.errors[0];
    assert!(e.message == "test", "error message should be `test`");

    let code = e
        .source
        .as_deref()
        .and_then(|e| e.downcast_ref::<GrandLineErr>())
        .unwrap_or_else(|| {
            let err = "downcast to GrandLineErr should be some";
            bug!(err)
        })
        .0
        .code();
    assert!(code == "Test", "error code should be `Test`");
}
