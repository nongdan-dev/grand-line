#[path = "../test_utils/mod.rs"]
mod test_utils;
use test_utils::*;

#[tokio::test]
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

        #[delete(User)]
        fn resolver() {}
    }
    use test::*;

    #[derive(Default, MergedObject)]
    struct Query(UserDetailQuery, UserSearchQuery, UserCountQuery);
    #[derive(Default, MergedObject)]
    struct Mutation(UserDeleteMutation);

    let _db = db_1(User).await?;
    let db = _db.as_ref();
    let s = schema_qm::<Query, Mutation>(db);

    let u1 = am_create!(User { name: "Olivia" }).insert(db).await?;
    let u2 = am_create!(User { name: "Peter" }).insert(db).await?;
    let _ = User::soft_delete_by_id(&u1.id)?.exec(db).await?;

    // ========================================================================
    // detail

    let q = r#"
    query test($id: ID!) {
        userDetail(id: $id) {
            name
        }
    }
    "#;
    let v1 = value!({
        "id": u1.id.clone(),
    });
    let expected = value!({
        "userDetail": null,
    });
    exec_assert(&s, q, Some(&v1), &expected).await?;

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

    // ========================================================================
    // delete

    let q = r#"
    mutation test($id: ID!) {
        userDelete(id: $id) {
            id
        }
    }
    "#;
    let expected = value!({
        "userDelete": {
            "id": u2.id.clone(),
        },
    });
    let v2 = value!({
        "id": u2.id.clone(),
    });
    exec_assert(&s, q, Some(&v2), &expected).await?;

    match User::find_by_id(&u2.id).one(db).await? {
        Some(u) => assert!(
            u.deleted_at != None,
            "it should have soft delete by default, found deleted_at=None",
        ),
        None => assert!(
            false,
            "it should have soft delete by default: found None returned from db",
        ),
    }

    let q = r#"
    mutation test($id: ID!) {
        userDelete(id: $id, permanent: true) {
            id
        }
    }
    "#;
    exec_assert(&s, q, Some(&v2), &expected).await?;

    match User::find_by_id(&u2.id).count(db).await? {
        count => assert!(
            count == 0,
            "it should delete permanently in db, found count={}",
            count,
        ),
    }

    Ok(())
}
