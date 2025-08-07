#[path = "../test_utils/mod.rs"]
mod test_utils;
use test_utils::prelude::*;

#[tokio::test]
#[cfg_attr(feature = "serial", serial)]
async fn default() -> Result<(), Box<dyn Error + Send + Sync>> {
    mod test {
        use super::*;

        #[model]
        pub struct User {
            #[has_many]
            pub emails: Email,
        }
        #[model]
        pub struct Email {
            pub address: String,
            pub user_id: String,
        }

        #[detail(User)]
        fn resolver() {}
    }
    use test::*;

    let db = db_2(User, Email).await?;
    let s = schema_q::<UserDetailQuery>(&db);

    let u = am_create!(User).insert(&db).await?;
    let _ = am_create!(Email {
        address: "email@example.com",
        user_id: u.id.clone(),
    })
    .insert(&db)
    .await?;

    let q = r#"
    query test($id: ID!) {
        userDetail(id: $id) {
            emails {
                address
            }
        }
    }
    "#;
    let v = value!({
        "id": u.id,
    });
    let expected = value!({
        "userDetail": {
            "emails": [{
                "address": "email@example.com",
            }],
        },
    });

    exec_assert(&s, q, Some(v), expected).await?;
    Ok(())
}
