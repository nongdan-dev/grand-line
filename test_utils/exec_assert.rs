use crate::prelude::*;

pub async fn exec_assert<Q, M, S>(s: &Schema<Q, M, S>, q: &str, v: Option<Value>, expected: &Value)
where
    Q: ObjectType + Default + 'static,
    M: ObjectType + Default + 'static,
    S: SubscriptionType + 'static,
{
    let res = exec_assert_ok(s, q, v).await;
    pretty_eq!(res.data, expected.clone());
}

pub async fn exec_assert_ok<Q, M, S>(s: &Schema<Q, M, S>, q: &str, v: Option<Value>) -> Response
where
    Q: ObjectType + Default + 'static,
    M: ObjectType + Default + 'static,
    S: SubscriptionType + 'static,
{
    let res = exec(s, q, v).await;
    assert!(res.errors.is_empty(), "{:#?}", res.errors);
    res
}

pub async fn exec_assert_err<Q, M, S, E>(s: &Schema<Q, M, S>, q: &str, v: Option<Value>, err: E)
where
    Q: ObjectType + Default + 'static,
    M: ObjectType + Default + 'static,
    S: SubscriptionType + 'static,
    E: GrandLineErrImpl,
{
    let res = exec(s, q, v).await;
    check_err(&res, err);
}

async fn exec<Q, M, S>(s: &Schema<Q, M, S>, q: &str, v: Option<Value>) -> Response
where
    Q: ObjectType + Default + 'static,
    M: ObjectType + Default + 'static,
    S: SubscriptionType + 'static,
{
    let mut req = Request::new(q);
    if let Some(v) = v {
        req = req.variables(Variables::from_value(v));
    }
    s.execute(req).await
}
