pub use grand_line::prelude::*;

#[tokio::test]
async fn t() -> Res<()> {
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

    let tmp = tmp_db!(User, Org, UserInOrg);
    let s = schema_q::<UserDetailQuery>(&tmp.db).finish();

    let u = am_create!(User).insert(&tmp.db).await?;
    let o = am_create!(Org { name: "Fringe" }).insert(&tmp.db).await?;
    am_create!(UserInOrg {
        user_id: u.id.clone(),
        org_id: o.id,
    })
    .insert(&tmp.db)
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

    exec_assert(&s, q, Some(&v), &expected).await;
    tmp.drop().await
}
