#[path = "../test_utils/mod.rs"]
mod test_utils;
use test_utils::*;

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

type MySchema = Schema<Query, Mutation, EmptySubscription>;
#[derive(Default, MergedObject)]
struct Query(UserDetailQuery, UserSearchQuery, UserCountQuery);
#[derive(Default, MergedObject)]
struct Mutation(UserDeleteMutation);

async fn prepare_test_data() -> Res<TestData> {
    let tmp = tmp_db_1(User).await?;
    let s = schema_qm::<Query, Mutation>(&tmp.db);

    let u1 = am_create!(User { name: "Olivia" }).insert(&tmp.db).await?;
    let u2 = am_create!(User { name: "Peter" }).insert(&tmp.db).await?;
    let _ = User::soft_delete_by_id(&u1.id)?.exec(&tmp.db).await?;

    Ok(TestData {
        tmp,
        s,
        id1: u1.id,
        id2: u2.id,
    })
}
struct TestData {
    tmp: TmpDb,
    s: MySchema,
    id1: String,
    id2: String,
}

#[tokio::test]
async fn detail() -> Res<()> {
    let d = prepare_test_data().await?;

    let q = r#"
    query test($id: ID!) {
        userDetail(id: $id) {
            name
        }
    }
    "#;
    let v1 = value!({
        "id": d.id1.clone(),
    });
    let expected = value!({
        "userDetail": null,
    });
    exec_assert(&d.s, q, Some(&v1), &expected).await;

    d.tmp.drop().await
}

#[tokio::test]
async fn detail_include_deleted() -> Res<()> {
    let d = prepare_test_data().await?;

    let q = r#"
    query test($id: ID!) {
        userDetail(id: $id, includeDeleted: true) {
            name
        }
    }
    "#;
    let v1 = value!({
        "id": d.id1.clone(),
    });
    let expected = value!({
        "userDetail": {
            "name": "Olivia",
        },
    });
    exec_assert(&d.s, q, Some(&v1), &expected).await;

    d.tmp.drop().await
}

#[tokio::test]
async fn search() -> Res<()> {
    let d = prepare_test_data().await?;

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
    exec_assert(&d.s, q, None, &expected).await;

    d.tmp.drop().await
}

#[tokio::test]
async fn search_filter_deleted_at() -> Res<()> {
    let d = prepare_test_data().await?;

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
    exec_assert(&d.s, q, None, &expected).await;

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
    exec_assert(&d.s, q, None, &expected).await;

    d.tmp.drop().await
}

#[tokio::test]
async fn search_include_deleted() -> Res<()> {
    let d = prepare_test_data().await?;

    let q = r#"
    query test {
        userSearch(orderBy: [NameAsc], includeDeleted: true) {
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
    exec_assert(&d.s, q, None, &expected).await;

    d.tmp.drop().await
}

#[tokio::test]
async fn count() -> Res<()> {
    let d = prepare_test_data().await?;

    let q = r#"
    query test {
        userCount
    }
    "#;
    let expected = value!({
        "userCount": 1,
    });
    exec_assert(&d.s, q, None, &expected).await;

    d.tmp.drop().await
}

#[tokio::test]
async fn count_filter_deleted_at() -> Res<()> {
    let d = prepare_test_data().await?;

    let q = r#"
    query test {
        userCount(filter: { deletedAt_ne: null })
    }
    "#;
    let expected = value!({
        "userCount": 1,
    });
    exec_assert(&d.s, q, None, &expected).await;

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
    exec_assert(&d.s, q, None, &expected).await;

    d.tmp.drop().await
}

#[tokio::test]
async fn count_include_deleted() -> Res<()> {
    let d = prepare_test_data().await?;

    let q = r#"
    query test {
        userCount(includeDeleted: true)
    }
    "#;
    let expected = value!({
        "userCount": 2,
    });
    exec_assert(&d.s, q, None, &expected).await;

    d.tmp.drop().await
}

#[tokio::test]
async fn soft_delete_by_default() -> Res<()> {
    let d = prepare_test_data().await?;

    let q = r#"
    mutation test($id: ID!) {
        userDelete(id: $id) {
            id
        }
    }
    "#;
    let v = value!({
        "id": d.id2.clone(),
    });
    let expected = value!({
        "userDelete": {
            "id": d.id2.clone(),
        },
    });
    exec_assert(&d.s, q, Some(&v), &expected).await;

    match User::find_by_id(&d.id2).one(&d.tmp.db).await? {
        Some(u) => assert!(
            u.deleted_at != None,
            "it should have soft delete by default, found deleted_at=None",
        ),
        None => assert!(
            false,
            "it should have soft delete by default, found None returned from db",
        ),
    }

    d.tmp.drop().await
}

#[tokio::test]
async fn delete_permanent() -> Res<()> {
    let d = prepare_test_data().await?;

    let q = r#"
    mutation test($id: ID!) {
        userDelete(id: $id, permanent: true) {
            id
        }
    }
    "#;
    let v = value!({
        "id": d.id2.clone(),
    });
    let expected = value!({
        "userDelete": {
            "id": d.id2.clone(),
        },
    });
    exec_assert(&d.s, q, Some(&v), &expected).await;

    match User::find_by_id(&d.id2).count(&d.tmp.db).await? {
        count => assert!(
            count == 0,
            "it should delete permanently in db, found count={}",
            count,
        ),
    }

    d.tmp.drop().await
}
