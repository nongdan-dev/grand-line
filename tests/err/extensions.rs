#[path = "../test_utils/mod.rs"]
mod test_utils;
use test_utils::*;

#[grand_line_err]
enum MyErr {
    #[error("should be exposed to the client")]
    #[client]
    Client,
    #[error("server error should not be exposed to the client")]
    Server,
}

#[derive(Default)]
struct Query;
#[Object]
impl Query {
    async fn client(&self) -> Res<i64> {
        Err(MyErr::Client)?;
        Ok(0)
    }
    async fn server(&self) -> Res<i64> {
        Err(MyErr::Server)?;
        Ok(0)
    }
    async fn std(&self) -> Result<i64, Box<dyn Error + Send + Sync>> {
        Err("any other error such as std should not be exposed to the client")?;
        Ok(0)
    }
}

#[tokio::test]
async fn should_only_expose_client_errors() -> Res<()> {
    let tmp = tmp_db().await?;
    let s = schema_q::<Query>(&tmp.db);

    check(&s, "{ client }", MyErr::Client).await;
    check(
        &s,
        "{ server }",
        GrandLineInternalGraphQLErr::InternalServer,
    )
    .await;
    check(&s, "{ std }", GrandLineInternalGraphQLErr::InternalServer).await;

    tmp.drop().await
}

async fn check<T>(s: &Schema<Query, EmptyMutation, EmptySubscription>, req: &str, err: T)
where
    T: GrandLineErrImpl,
{
    let r = s.execute(req).await;
    assert!(r.errors.len() == 1, "response should have an error");

    let e = &r.errors[0];
    let expected_message = err.to_string();
    assert!(
        e.message == expected_message,
        "error message should match `{}`",
        expected_message,
    );

    if let Some(extensions) = e.extensions.as_ref() {
        let expected_code = err.code();
        if let Some(Value::String(code)) = extensions.get("code")
            && code == expected_code
        {
            // ok
        } else {
            assert!(
                false,
                "error extensions code should match `{}`",
                expected_code,
            );
        }
    } else {
        assert!(false, "error extensions should be some");
    }
}
