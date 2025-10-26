#[path = "../test_utils/mod.rs"]
mod test_utils;
use test_utils::*;

#[tokio::test]
async fn t() -> Res<()> {
    mod test {
        use super::*;

        #[model]
        pub struct User {
            pub a: i64,
            #[sql_expr(Expr::col(Column::A).add(1000))]
            pub b: i64,
        }

        #[detail(User)]
        fn resolver() {}
    }
    use test::*;

    let tmp = tmp_db!(User);
    let s = schema_q::<UserDetailQuery>(&tmp.db);

    let u = db_create!(&tmp.db, User { a: 1 });

    let q = r#"
    query test($id: ID!) {
        userDetail(id: $id) {
            b
        }
    }
    "#;
    let v = value!({
        "id": u.id,
    });
    let expected = value!({
        "userDetail": {
            "b": 1001,
        },
    });

    exec_assert(&s, q, Some(&v), &expected).await;
    tmp.drop().await
}
