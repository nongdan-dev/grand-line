pub use grand_line::prelude::*;

#[tokio::test]
async fn t() -> Res<()> {
    mod test {
        use super::*;

        #[model]
        pub struct User {
            #[has_many]
            pub aliases: Alias,
        }
        #[model]
        pub struct Alias {
            pub name: String,
            pub user_id: String,
        }

        #[detail(User)]
        fn resolver() {}
    }
    use test::*;

    let tmp = tmp_db!(User, Alias);
    let s = schema_q::<UserDetailQuery>(&tmp.db).finish();

    let u = am_create!(User).insert(&tmp.db).await?;
    am_create!(Alias {
        name: "Liv",
        user_id: u.id.clone(),
    })
    .insert(&tmp.db)
    .await?;

    let q = r#"
    query test($id: ID!) {
        userDetail(id: $id) {
            aliases {
                name
            }
        }
    }
    "#;
    let v = value!({
        "id": u.id,
    });
    let expected = value!({
        "userDetail": {
            "aliases": [{
                "name": "Liv",
            }],
        },
    });

    exec_assert(&s, q, Some(&v), &expected).await;
    tmp.drop().await
}
