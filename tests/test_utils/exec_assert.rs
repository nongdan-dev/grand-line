use super::prelude::*;

pub async fn exec_assert<Q, M, S>(
    s: Schema<Q, M, S>,
    q: &str,
    v: Value,
    expected: Value,
) -> Result<(), Box<dyn Error>>
where
    Q: ObjectType + Default + 'static,
    M: ObjectType + Default + 'static,
    S: SubscriptionType + 'static,
{
    let req = Request::new(q).variables(Variables::from_value(v));
    let res = s.execute(req).await;
    assert!(res.errors.is_empty(), "{:#?}", res.errors);
    pretty_eq!(res.data, expected);
    Ok(())
}
