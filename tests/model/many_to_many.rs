#[path = "../test_utils/mod.rs"]
mod test_utils;
use test_utils::prelude::*;

#[model]
pub struct User {
    #[many_to_many]
    pub orgs: Org,
}
#[model]
pub struct Org {
    pub name: String,
}
#[model]
pub struct UserInOrg {
    pub user_id: String,
    pub org_id: String,
}
#[detail(User)]
fn resolver() {}

#[tokio::test]
#[cfg_attr(feature = "serial_db", serial(db))]
async fn default() -> Result<(), Box<dyn Error>> {
    let db = db_3(User, Org, UserInOrg).await?;
    let gql = schema_q::<UserDetailQuery>(&db);
    let u = active_create!(User {}).insert(&db).await?;
    let o = active_create!(Org { name: "Fringe" }).insert(&db).await?;
    let _ = active_create!(UserInOrg {
        user_id: u.id.clone(),
        org_id: o.id.clone(),
    })
    .insert(&db)
    .await?;

    let q = r#"
    query test($id: ID!) {
        userDetail(id: $id) {
            orgs {
                name
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
            "orgs": [{
                "name": "Fringe",
            }],
        },
    });
    pretty_eq!(res.data, expected);

    Ok(())
}
