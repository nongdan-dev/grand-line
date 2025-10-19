#[path = "../test_utils/mod.rs"]
mod test_utils;
use test_utils::*;

#[derive(ThisError, Debug, Clone)]
enum MyErr {
    #[error("test")]
    Test,
}
type MyRes<T> = Result<T, MyErr>;

struct MyQuery;
#[Object]
impl MyQuery {
    async fn my_err(&self) -> MyRes<i64> {
        Err(MyErr::Test)?;
        Ok(0)
    }
    async fn my_err_dyn(&self) -> Result<i64, Box<dyn Error + Send + Sync>> {
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

    let my = e.source.as_deref().and_then(|e| e.downcast_ref::<MyErr>());

    assert!(my.is_some(), "downcast to MyErr should be some");
    assert!(matches!(my.unwrap(), MyErr::Test), "should be MyErr::Test");
}

#[tokio::test]
async fn dyn_should_be_my_err() {
    let s = schema();

    let r = s.execute("{ myErrDyn }").await;
    assert!(r.errors.len() == 1, "response should have an error");

    let e = &r.errors[0];
    assert!(e.message == "test", "error message should match");

    let box_dyn = e
        .source
        .as_deref()
        .and_then(|e| e.downcast_ref::<Box<dyn Error + Send + Sync>>());
    assert!(box_dyn.is_some(), "downcast to Box dyn should be some");

    let my = box_dyn.unwrap().downcast_ref::<MyErr>();
    assert!(my.is_some(), "downcast to MyErr should be some");
    assert!(matches!(my.unwrap(), MyErr::Test), "should be MyErr::Test");
}
