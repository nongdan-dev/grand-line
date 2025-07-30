#[path = "../test_utils/mod.rs"]
mod test_utils;
use test_utils::prelude::*;

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

#[tokio::test]
#[cfg_attr(feature = "serial", serial)]
async fn default() -> Result<(), Box<dyn Error>> {
    let db = db_2(User, Profile).await?;
    let u = am_create!(User { name: "Olivia" }).insert(&db).await?;
    let f = am_create!(Profile { user_id: u.id }).insert(&db).await?;

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

    let s = schema_q::<ProfileDetailQuery>(&db);
    exec_assert(s, q, v, expected).await?;
    Ok(())
}
