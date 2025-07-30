#[path = "../test_utils/mod.rs"]
mod test_utils;
use test_utils::prelude::*;

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

#[tokio::test]
#[cfg_attr(feature = "serial_db", serial(db))]
async fn default() -> Result<(), Box<dyn Error>> {
    let db = db_2(User, Email).await?;
    let gql = schema_q::<UserDetailQuery>(&db);
    let u = active_create!(User {}).insert(&db).await?;
    let _ = active_create!(Email {
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
    let req = request(q, v);
    let res = gql.execute(req).await;
    assert!(res.errors.is_empty(), "{:#?}", res.errors);

    let expected = value!({
        "userDetail": {
            "emails": [{
                "address": "email@example.com",
            }],
        },
    });
    pretty_eq!(res.data, expected);

    Ok(())
}
