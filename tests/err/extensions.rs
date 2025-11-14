pub use grand_line::prelude::*;

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
    let s = schema_q::<Query>(&tmp.db).finish();

    check(&s, "{ client }", MyErr::Client).await;
    check(&s, "{ server }", GrandLineGraphQLErr::InternalServer).await;
    check(&s, "{ std }", GrandLineGraphQLErr::InternalServer).await;

    tmp.drop().await
}

async fn check<T>(s: &Schema<Query, EmptyMutation, EmptySubscription>, req: &str, err: T)
where
    T: GqlErrImpl,
{
    let r = s.execute(req).await;
    check_err(&r, err);
}
