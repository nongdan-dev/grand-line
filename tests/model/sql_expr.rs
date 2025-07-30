#[path = "../test_utils/mod.rs"]
mod test_utils;
use test_utils::prelude::*;

#[model]
pub struct Data {
    a: i64,
    #[sql_expr(Expr::col(Column::A).add(1000))]
    b: i64,
}
#[detail(Data)]
fn sqlExpr() {}

#[tokio::test]
#[cfg_attr(feature = "serial_db", serial(db))]
async fn default() -> Result<(), Box<dyn Error>> {
    let db = db_1(Data).await?;
    let gql = schema_q::<SqlExprQuery>(&db);
    let d = active_create!(Data { a: 1 }).insert(&db).await?;

    let q = r#"
    query test($id: ID!) {
        sqlExpr(id: $id) {
            b
        }
    }
    "#;
    let v = value!({
        "id": d.id,
    });
    let req = request(q, v);
    let res = gql.execute(req).await;
    assert!(res.errors.is_empty(), "{:#?}", res.errors);

    let expected = value!({
        "sqlExpr": {
            "b": 1001,
        },
    });
    pretty_eq!(res.data, expected);

    Ok(())
}
