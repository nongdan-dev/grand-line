pub use grand_line::prelude::*;

#[tokio::test]
async fn t() -> Res<()> {
    mod test {
        use super::*;

        #[model]
        pub struct User {
            pub name: String,
        }
        #[model]
        pub struct Alias {
            pub user_id: String,
            #[belongs_to]
            pub user: User,
        }

        #[detail(Alias)]
        fn resolver() {}
    }
    use test::*;

    let tmp = tmp_db!(User, Alias);
    let s = schema_q::<AliasDetailQuery>(&tmp.db).finish();

    let u = am_create!(User { name: "Olivia" }).insert(&tmp.db).await?;
    let f = am_create!(Alias { user_id: u.id }).insert(&tmp.db).await?;

    let q = r#"
    query test($id: ID!) {
        aliasDetail(id: $id) {
            user {
                name
            }
        }
    }
    "#;
    let v = value!({
        "id": f.id,
    });
    let expected = value!({
        "aliasDetail": {
            "user": {
                "name": "Olivia",
            },
        },
    });

    exec_assert(&s, q, Some(&v), &expected).await;
    tmp.drop().await
}
