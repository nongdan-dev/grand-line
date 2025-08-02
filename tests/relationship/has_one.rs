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
            #[has_one]
            pub profile: Profile,
        }
        #[model]
        pub struct Profile {
            pub gender: String,
            pub user_id: String,
        }

        #[detail(User)]
        fn resolver() {}
    }
    use test::*;

    let db = db_2(User, Profile).await?;
    let u = am_create!(User).insert(&db).await?;
    let _ = am_create!(Profile {
        gender: "Binary",
        user_id: u.id.clone(),
    })
    .insert(&db)
    .await?;

    let q = r#"
    query test($id: ID!) {
        userDetail(id: $id) {
            profile {
                gender
            }
        }
    }
    "#;
    let v = value!({
        "id": u.id,
    });
    let expected = value!({
        "userDetail": {
            "profile": {
                "gender": "Binary",
            },
        },
    });

    let s = schema_q::<UserDetailQuery>(&db);
    exec_assert(s, q, v, expected).await?;
    Ok(())
}
