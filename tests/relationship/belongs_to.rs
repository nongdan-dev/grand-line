#[path = "../test_utils/mod.rs"]
mod test_utils;
use test_utils::*;

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

    let u = db_create!(&tmp.db, User { name: "Olivia" });
    let f = db_create!(&tmp.db, Alias { user_id: u.id });

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
