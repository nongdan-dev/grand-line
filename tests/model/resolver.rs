pub use grand_line::prelude::*;

#[tokio::test]
async fn sql_dep_cols() -> Res<()> {
    mod test {
        use super::*;

        #[model]
        pub struct User {
            pub first_name: String,
            #[graphql(skip)]
            pub last_name: String,
            #[resolver(sql_dep = "first_name, last_name")]
            pub full_name: String,
        }

        async fn resolve_full_name(u: &UserGql, _: &Context<'_>) -> Res<String> {
            let full_name = vec![
                u.first_name.clone().ok_or(CoreDbErr::GqlResolverNone)?,
                u.last_name.clone().ok_or(CoreDbErr::GqlResolverNone)?,
            ]
            .join(" ");
            Ok(full_name)
        }

        #[detail(User)]
        fn resolver() {}
    }
    use test::*;

    let tmp = tmp_db!(User);
    let s = schema_q::<UserDetailQuery>(&tmp.db).finish();

    let u = am_create!(User {
        first_name: "Olivia",
        last_name: "Dunham",
    })
    .insert(&tmp.db)
    .await?;

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
            "fullName": "Olivia Dunham",
        },
    });

    exec_assert(&s, q, Some(v), &expected).await;
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
            #[resolver(sql_dep = "a, b")]
            c: i64,
        }

        async fn resolve_c(u: &UserGql, _: &Context<'_>) -> Res<i64> {
            let a = u.a.ok_or(CoreDbErr::GqlResolverNone)?;
            let b = u.b.ok_or(CoreDbErr::GqlResolverNone)?;
            Ok(a + b)
        }

        #[detail(User)]
        fn resolver() {}
    }
    use test::*;

    let tmp = tmp_db!(User);
    let s = schema_q::<UserDetailQuery>(&tmp.db).finish();

    let u = am_create!(User { a: 1 }).insert(&tmp.db).await?;

    let q = r#"
    query test($id: ID!) {
        userDetail(id: $id) {
            c
        }
    }
    "#;
    let v = value!({
        "id": u.id,
    });
    let expected = value!({
        "userDetail": {
            "c": 1002,
        },
    });

    exec_assert(&s, q, Some(v), &expected).await;
    tmp.drop().await
}
