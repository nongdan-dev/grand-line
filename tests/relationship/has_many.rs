#[path = "../test_utils/mod.rs"]
mod test_utils;
use test_utils::*;

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
    let s = schema_q::<UserDetailQuery>(&tmp.db);

    let u = db_create!(&tmp.db, User);
    let _ = db_create!(
        &tmp.db,
        Alias {
            name: "Liv",
            user_id: u.id.clone(),
        },
    );

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
