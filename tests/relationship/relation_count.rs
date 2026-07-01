pub use grand_line::prelude::*;

#[tokio::test]
async fn has_many_count() -> Res<()> {
    mod test {
        use super::*;

        #[model]
        pub struct User {
            #[has_many(count)]
            pub aliases: Alias,
        }
        #[model]
        pub struct Alias {
            pub name: String,
            pub user_id: String,
        }

        #[detail(User)]
        fn resolver() {
        }
    }
    use test::*;

    let tmp = tmp_db!(User, Alias);
    let s = schema_q::<UserDetailQuery>(&tmp.db).finish();

    let u1 = am_create!(User).exec_without_ctx(&tmp.db).await?;
    let u2 = am_create!(User).exec_without_ctx(&tmp.db).await?;
    am_create!(Alias {
        name: "Liv",
        user_id: u1.id.clone(),
    })
    .exec_without_ctx(&tmp.db)
    .await?;
    am_create!(Alias {
        name: "Fauxlivia",
        user_id: u1.id.clone(),
    })
    .exec_without_ctx(&tmp.db)
    .await?;

    let q = "
    query test($id: ID!) {
        userDetail(id: $id) {
            aliases_count
        }
    }
    ";
    let v = value!({
        "id": u1.id,
    });
    let expected = value!({
        "userDetail": {
            "aliases_count": 2,
        },
    });
    exec_assert(&s, q, Some(v), &expected).await;

    let v = value!({
        "id": u2.id,
    });
    let expected = value!({
        "userDetail": {
            "aliases_count": 0,
        },
    });
    exec_assert(&s, q, Some(v), &expected).await;

    let q = r#"
    query test($id: ID!) {
        userDetail(id: $id) {
            aliases_count(filter: { name: "Liv" })
        }
    }
    "#;
    let v = value!({
        "id": u1.id,
    });
    let expected = value!({
        "userDetail": {
            "aliases_count": 1,
        },
    });
    exec_assert(&s, q, Some(v), &expected).await;

    tmp.drop().await
}

#[tokio::test]
async fn many_to_many_count() -> Res<()> {
    mod test {
        use super::*;

        #[model]
        pub struct User {
            #[many_to_many(count)]
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
        fn resolver() {
        }
    }
    use test::*;

    let tmp = tmp_db!(User, Org, UserInOrg);
    let s = schema_q::<UserDetailQuery>(&tmp.db).finish();

    let u = am_create!(User).exec_without_ctx(&tmp.db).await?;
    let o1 = am_create!(Org {
        name: "Fringe",
    })
    .exec_without_ctx(&tmp.db)
    .await?;
    let o2 = am_create!(Org {
        name: "FBI",
    })
    .exec_without_ctx(&tmp.db)
    .await?;
    am_create!(UserInOrg {
        user_id: u.id.clone(),
        org_id: o1.id,
    })
    .exec_without_ctx(&tmp.db)
    .await?;
    am_create!(UserInOrg {
        user_id: u.id.clone(),
        org_id: o2.id,
    })
    .exec_without_ctx(&tmp.db)
    .await?;

    let q = "
    query test($id: ID!) {
        userDetail(id: $id) {
            orgs_count
        }
    }
    ";
    let v = value!({
        "id": u.id,
    });
    let expected = value!({
        "userDetail": {
            "orgs_count": 2,
        },
    });
    exec_assert(&s, q, Some(v), &expected).await;

    let q = r#"
    query test($id: ID!) {
        userDetail(id: $id) {
            orgs_count(filter: { name: "Fringe" })
        }
    }
    "#;
    let v = value!({
        "id": u.id,
    });
    let expected = value!({
        "userDetail": {
            "orgs_count": 1,
        },
    });
    exec_assert(&s, q, Some(v), &expected).await;

    tmp.drop().await
}
