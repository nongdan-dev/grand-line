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
    }
    use test::*;

    let db = db_3(User, Org, UserInOrg).await?;
    let u = am_create!(User).insert(&db).await?;
    let o = am_create!(Org { name: "Fringe" }).insert(&db).await?;
    let _ = am_create!(UserInOrg {
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
    let expected = value!({
        "userDetail": {
            "orgs": [{
                "name": "Fringe",
            }],
        },
    });

    let s = schema_q::<UserDetailQuery>(&db);
    exec_assert(s, q, v, expected).await?;
    Ok(())
}
