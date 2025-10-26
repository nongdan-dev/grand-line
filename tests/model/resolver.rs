#[path = "../test_utils/mod.rs"]
mod test_utils;
use test_utils::*;

#[tokio::test]
async fn sql_dep_cols() -> Res<()> {
    mod test {
        use super::*;

        #[model]
        pub struct User {
            pub first_name: String,
            pub middle_name: String,
            pub last_name: String,
            #[resolver(sql_dep=first_name+middle_name+last_name)]
            pub full_name: String,
        }

        async fn resolve_full_name(u: &UserGql, _: &Context<'_>) -> Res<String> {
            let err = "should be selected from database already";
            let full_name = vec![
                u.first_name.clone().unwrap_or_else(|| bug!(err)),
                u.middle_name.clone().unwrap_or_else(|| bug!(err)),
                u.last_name.clone().unwrap_or_else(|| bug!(err)),
            ]
            .join(" ");
            Ok(full_name)
        }

        #[detail(User)]
        fn resolver() {}
    }
    use test::*;

    let tmp = tmp_db!(User);
    let s = schema_q::<UserDetailQuery>(&tmp.db);

    let u = db_create!(
        &tmp.db,
        User {
            first_name: "Olivia",
            middle_name: "Anna",
            last_name: "Dunham",
        },
    );

    let q = r#"
    query test($id: ID!) {
        userDetail(id: $id) {
            fullName
        }
    }
    "#;
    let v = value!({
        "id": u.id,
    });
    let expected = value!({
        "userDetail": {
            "fullName": "Olivia Anna Dunham",
        },
    });

    exec_assert(&s, q, Some(&v), &expected).await;
    tmp.drop().await
}

#[tokio::test]
async fn sql_dep_exprs() -> Res<()> {
    mod test {
        use super::*;

        #[model]
        pub struct User {
            a: i64,
            #[sql_expr(Expr::col(Column::A).add(1000))]
            b: i64,
            #[sql_expr(Expr::col(Column::A).add(2000))]
            c: i64,
            #[resolver(sql_dep=a+b+c)]
            d: i64,
        }

        async fn resolve_d(u: &UserGql, _: &Context<'_>) -> Res<i64> {
            let err = "should be selected from database already";
            let a = u.a.unwrap_or_else(|| bug!(err));
            let b = u.b.unwrap_or_else(|| bug!(err));
            let c = u.c.unwrap_or_else(|| bug!(err));
            let d = a + b + c;
            Ok(d)
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
            d
        }
    }
    "#;
    let v = value!({
        "id": u.id,
    });
    let expected = value!({
        "userDetail": {
            "d": 3003,
        },
    });

    exec_assert(&s, q, Some(&v), &expected).await;
    tmp.drop().await
}
