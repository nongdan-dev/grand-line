#[path = "../test_utils/mod.rs"]
mod test_utils;
use test_utils::*;

#[tokio::test]
#[cfg_attr(feature = "serial", serial)]
async fn default() -> Result<(), Box<dyn Error + Send + Sync>> {
    mod test {
        use super::*;

        #[model]
        pub struct User {
            pub name: String,
        }

        #[detail(User)]
        fn resolver() {}

        #[search(User)]
        fn resolver() {
            (None, None)
        }
        #[count(User)]
        fn resolver() {
            None
        }
    }
    use test::*;

    #[derive(Default, MergedObject)]
    struct Query(UserDetailQuery, UserSearchQuery, UserCountQuery);

    let db = db_1(User).await?;
    let s = schema_q::<Query>(&db);

    let _ = am_create!(User { name: "Peter" }).insert(&db).await?;
    let u = am_create!(User { name: "Olivia" }).insert(&db).await?;
    let u = u.into_active_model().soft_delete(&db).await?;

    // ========================================================================
    // detail

    let q = r#"
    query test($id: ID!) {
        userDetail(id: $id) {
            name
        }
    }
    "#;
    let v = value!({
        "id": u.id,
    });
    let expected = value!({
        "userDetail": null,
    });
    exec_assert(&s, q, Some(&v), &expected).await?;

    // ========================================================================
    // search

    let q = r#"
    query test {
        userSearch {
            name
        }
    }
    "#;
    let expected = value!({
        "userSearch": [{
            "name": "Peter",
        }],
    });
    exec_assert(&s, q, None, &expected).await?;

    let q = r#"
    query test {
        userSearch(filter: { deletedAt_ne: null }) {
            name
        }
    }
    "#;
    let expected = value!({
        "userSearch": [{
            "name": "Olivia",
        }],
    });
    exec_assert(&s, q, None, &expected).await?;

    let q = r#"
    query test {
        userSearch(
            filter: {
                OR: [
                    { deletedAt: null },
                    { deletedAt_ne: null },
                ],
            },
            orderBy: [NameAsc],
        ) {
            name
        }
    }
    "#;
    let expected = value!({
        "userSearch": [{
            "name": "Olivia",
        }, {
            "name": "Peter",
        }],
    });
    exec_assert(&s, q, None, &expected).await?;

    let q = r#"
    query test {
        userSearch(orderBy: [NameAsc], includeDeleted: true) {
            name
        }
    }
    "#;
    exec_assert(&s, q, None, &expected).await?;

    // ========================================================================
    // count

    let q = r#"
    query test {
        userCount
    }
    "#;
    let expected = value!({
        "userCount": 1,
    });
    exec_assert(&s, q, None, &expected).await?;

    let q = r#"
    query test {
        userCount(filter: { deletedAt_ne: null })
    }
    "#;
    exec_assert(&s, q, None, &expected).await?;

    let q = r#"
    query test {
        userCount(
            filter: {
                OR: [
                    { deletedAt: null },
                    { deletedAt_ne: null },
                ],
            },
        )
    }
    "#;
    let expected = value!({
        "userCount": 2,
    });
    exec_assert(&s, q, None, &expected).await?;

    let q = r#"
    query test {
        userCount(includeDeleted: true)
    }
    "#;
    exec_assert(&s, q, None, &expected).await?;

    Ok(())
}
