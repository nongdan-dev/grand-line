#![allow(dead_code)]

use super::*;

pub async fn exec_assert<Q, M, S>(
    s: &Schema<Q, M, S>,
    q: &str,
    v: Option<&Value>,
    expected: &Value,
) -> Result<(), Box<dyn Error + Send + Sync>>
where
    Q: ObjectType + Default + 'static,
    M: ObjectType + Default + 'static,
    S: SubscriptionType + 'static,
{
    let mut req = Request::new(q);
    if let Some(v) = v.cloned() {
        req = req.variables(Variables::from_value(v));
    }
    let res = s.execute(req).await;
    assert!(res.errors.is_empty(), "{:#?}", res.errors);
    pretty_eq!(res.data, expected.clone());
    Ok(())
}
