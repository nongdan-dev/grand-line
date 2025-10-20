#[path = "../test_utils/mod.rs"]
mod test_utils;
use test_utils::*;

#[tokio::test]
async fn default() -> Res<()> {
    mod test {
        use super::*;

        #[model]
        pub struct User {
            pub name: String,
        }
        #[model]
        pub struct Profile {
            pub user_id: String,
            #[belongs_to]
            pub user: User,
        }

        #[detail(Profile)]
        fn resolver() {}
    }
    use test::*;

    let tmp = tmp_db_2(User, Profile).await?;
    let s = schema_q::<ProfileDetailQuery>(&tmp.db);

    let u = am_create!(User { name: "Olivia" }).insert(&tmp.db).await?;
    let f = am_create!(Profile { user_id: u.id })
        .insert(&tmp.db)
        .await?;

    let q = r#"
    query test($id: ID!) {
        profileDetail(id: $id) {
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
        "profileDetail": {
            "user": {
                "name": "Olivia",
            },
        },
    });

    exec_assert(&s, q, Some(&v), &expected).await;
    tmp.drop().await
}
