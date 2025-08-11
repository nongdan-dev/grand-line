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
    }
    use test::*;

    #[derive(Default, MergedObject)]
    struct Query(UserDetailQuery, UserSearchQuery);

    let db = db_1(User).await?;
    let s = schema_q::<Query>(&db);

    let u = am_create!(User { name: "Olivia" }).insert(&db).await?;
    let u = u.into_active_model().soft_delete(&db).await?;

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

    exec_assert(&s, q, Some(v), expected).await?;

    let q = r#"
    query test {
        userSearch {
            name
        }
    }
    "#;
    let expected = value!({
        "userSearch": [],
    });

    exec_assert(&s, q, None, expected).await?;

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

    exec_assert(&s, q, None, expected).await?;
    Ok(())
}
