#[path = "../test_utils/mod.rs"]
mod test_utils;
use test_utils::prelude::*;

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

#[tokio::test]
#[cfg_attr(feature = "serial_db", serial(db))]
async fn default() -> Result<(), Box<dyn Error>> {
    let db = db_2(User, Profile).await?;
    let gql = schema_q::<UserDetailQuery>(&db);
    let u = active_create!(User {}).insert(&db).await?;
    let _ = active_create!(Profile {
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
    let req = request(q, v);
    let res = gql.execute(req).await;
    assert!(res.errors.is_empty(), "{:#?}", res.errors);

    let expected = value!({
        "userDetail": {
            "profile": {
                "gender": "Binary",
            },
        },
    });
    pretty_eq!(res.data, expected);

    Ok(())
}
