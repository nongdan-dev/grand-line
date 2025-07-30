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
fn resolver() {}

#[tokio::test]
#[cfg_attr(feature = "serial", serial)]
async fn default() -> Result<(), Box<dyn Error>> {
    let db = db_1(Data).await?;
    let d = am_create!(Data { a: 1 }).insert(&db).await?;

    let q = r#"
    query test($id: ID!) {
        dataDetail(id: $id) {
            b
        }
    }
    "#;
    let v = value!({
        "id": d.id,
    });
    let expected = value!({
        "dataDetail": {
            "b": 1001,
        },
    });

    let s = schema_q::<DataDetailQuery>(&db);
    exec_assert(s, q, v, expected).await?;
    Ok(())
}
