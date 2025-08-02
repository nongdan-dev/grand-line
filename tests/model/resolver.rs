#[path = "../test_utils/mod.rs"]
mod test_utils;
use test_utils::prelude::*;

#[tokio::test]
#[cfg_attr(feature = "serial", serial)]
async fn default() -> Result<(), Box<dyn Error>> {
    mod test {
        use super::*;

        #[model]
        pub struct User {
            first_name: String,
            middle_name: String,
            last_name: String,
            #[resolver(sql_dep=first_name+middle_name+last_name)]
            full_name: String,
        }

        async fn resolve_full_name(
            u: &UserGql,
            _: &Context<'_>,
        ) -> Result<String, Box<dyn Error + Send + Sync>> {
            let err = "should be selected from database already";
            let full_name = vec![
                u.first_name.clone().ok_or_else(|| err)?,
                u.middle_name.clone().ok_or_else(|| err)?,
                u.last_name.clone().ok_or_else(|| err)?,
            ]
            .join(" ");
            Ok(full_name)
        }

        #[detail(User)]
        fn resolver() {}
    }
    use test::*;

    let db = db_1(User).await?;
    let d = am_create!(User {
        first_name: "Olivia",
        middle_name: "Anna",
        last_name: "Dunham",
    })
    .insert(&db)
    .await?;

    let q = r#"
    query test($id: ID!) {
        userDetail(id: $id) {
            fullName
        }
    }
    "#;
    let v = value!({
        "id": d.id,
    });
    let expected = value!({
        "userDetail": {
            "fullName": "Olivia Anna Dunham",
        },
    });

    let s = schema_q::<UserDetailQuery>(&db);
    exec_assert(s, q, v, expected).await?;
    Ok(())
}

#[tokio::test]
#[cfg_attr(feature = "serial", serial)]
async fn sql_expr() -> Result<(), Box<dyn Error>> {
    mod test {
        use super::*;

        #[model]
        pub struct Data {
            a: i64,
            #[sql_expr(Expr::col(Column::A).add(1000))]
            b: i64,
            #[sql_expr(Expr::col(Column::A).add(2000))]
            c: i64,
            #[resolver(sql_dep=a+b+c)]
            d: i64,
        }

        async fn resolve_d(
            u: &DataGql,
            _: &Context<'_>,
        ) -> Result<i64, Box<dyn Error + Send + Sync>> {
            let err = "should be selected from database already";
            let a = u.a.ok_or_else(|| err)?;
            let b = u.b.ok_or_else(|| err)?;
            let c = u.c.ok_or_else(|| err)?;
            let d = a + b + c;
            Ok(d)
        }

        #[detail(Data)]
        fn resolver() {}
    }
    use test::*;

    let db = db_1(Data).await?;
    let d = am_create!(Data { a: 1 }).insert(&db).await?;

    let q = r#"
    query test($id: ID!) {
        dataDetail(id: $id) {
            d
        }
    }
    "#;
    let v = value!({
        "id": d.id,
    });
    let expected = value!({
        "dataDetail": {
            "d": 3003,
        },
    });

    let s = schema_q::<DataDetailQuery>(&db);
    exec_assert(s, q, v, expected).await?;
    Ok(())
}
