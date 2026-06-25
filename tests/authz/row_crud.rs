#[path = "./row_crud_setup.rs"]
mod row_crud_setup;
use row_crud_setup::*;

// ---------------------------------------------------------------------------
// search tests
// ---------------------------------------------------------------------------

// No row_policy entry -> all tasks returned.
#[tokio::test]
async fn search_no_policy() -> Res<()> {
    let d = row_crud_setup(RowPolicy::default(), AuthzConfig::default()).await?;

    let q = "
    query {
        taskSearch(orderBy: [TitleAsc]) {
            title
        }
    }
    ";
    let expected = value!({
        "taskSearch": [{
            "title": "Task1",
        }, {
            "title": "Task2",
        }],
    });
    exec_assert(&d.schema, q, None, &expected).await;

    d.tmp.drop().await
}

// OrgHandler returns org1 filter -> only Task1 (org1) returned.
#[tokio::test]
async fn search_with_filter() -> Res<()> {
    let pol = row_policy("taskSearch".to_owned(), "any".to_owned());
    let cfg = AuthzConfig {
        handlers: Arc::new(OrgHandler),
        ..Default::default()
    };
    let d = row_crud_setup(pol, cfg).await?;

    let q = "
    query {
        taskSearch(orderBy: [TitleAsc]) {
            title
        }
    }
    ";
    let expected = value!({
        "taskSearch": [{
            "title": "Task1",
        }],
    });
    exec_assert(&d.schema, q, None, &expected).await;

    d.tmp.drop().await
}

// ---------------------------------------------------------------------------
// count tests
// ---------------------------------------------------------------------------

// No row_policy entry -> both tasks counted.
#[tokio::test]
async fn count_no_policy() -> Res<()> {
    let d = row_crud_setup(RowPolicy::default(), AuthzConfig::default()).await?;

    let q = "
    query {
        taskCount
    }
    ";
    let expected = value!({
        "taskCount": 2,
    });
    exec_assert(&d.schema, q, None, &expected).await;

    d.tmp.drop().await
}

// OrgHandler returns org1 filter -> only Task1 counted.
#[tokio::test]
async fn count_with_filter() -> Res<()> {
    let pol = row_policy("taskCount".to_owned(), "any".to_owned());
    let cfg = AuthzConfig {
        handlers: Arc::new(OrgHandler),
        ..Default::default()
    };
    let d = row_crud_setup(pol, cfg).await?;

    let q = "
    query {
        taskCount
    }
    ";
    let expected = value!({
        "taskCount": 1,
    });
    exec_assert(&d.schema, q, None, &expected).await;

    d.tmp.drop().await
}

// ---------------------------------------------------------------------------
// detail tests
// ---------------------------------------------------------------------------

// No row_policy entry -> task detail returned without filter.
#[tokio::test]
async fn detail_no_policy() -> Res<()> {
    let d = row_crud_setup(RowPolicy::default(), AuthzConfig::default()).await?;

    let q = "
    query($id: ID!) {
        taskDetail(id: $id) {
            title
        }
    }
    ";
    let v = value!({
        "id": d.task1_id,
    });
    let expected = value!({
        "taskDetail": {
            "title": "Task1",
        },
    });
    exec_assert(&d.schema, q, Some(v), &expected).await;

    d.tmp.drop().await
}

// OrgHandler returns org1 filter; task1 belongs to org1 -> returned.
#[tokio::test]
async fn detail_filter_match() -> Res<()> {
    let pol = row_policy("taskDetail".to_owned(), "any".to_owned());
    let cfg = AuthzConfig {
        handlers: Arc::new(OrgHandler),
        ..Default::default()
    };
    let d = row_crud_setup(pol, cfg).await?;

    let q = "
    query($id: ID!) {
        taskDetail(id: $id) {
            title
        }
    }
    ";
    let v = value!({
        "id": d.task1_id,
    });
    let expected = value!({
        "taskDetail": {
            "title": "Task1",
        },
    });
    exec_assert(&d.schema, q, Some(v), &expected).await;

    d.tmp.drop().await
}

// OrgHandler returns org1 filter; task2 belongs to org2 -> null.
#[tokio::test]
async fn detail_filter_no_match() -> Res<()> {
    let pol = row_policy("taskDetail".to_owned(), "any".to_owned());
    let cfg = AuthzConfig {
        handlers: Arc::new(OrgHandler),
        ..Default::default()
    };
    let d = row_crud_setup(pol, cfg).await?;

    let q = "
    query($id: ID!) {
        taskDetail(id: $id) {
            title
        }
    }
    ";
    let v = value!({
        "id": d.task2_id,
    });
    let expected = value!({
        "taskDetail": null,
    });
    exec_assert(&d.schema, q, Some(v), &expected).await;

    d.tmp.drop().await
}

// ---------------------------------------------------------------------------
// delete tests
// ---------------------------------------------------------------------------

// No row_policy entry -> delete succeeds without filter.
// gql_delete returns G::from_id so only id is populated in the response.
#[tokio::test]
async fn delete_no_policy() -> Res<()> {
    let d = row_crud_setup(RowPolicy::default(), AuthzConfig::default()).await?;

    let q = "
    mutation($id: ID!) {
        taskDelete(id: $id, permanent: true) {
            id
        }
    }
    ";
    let v = value!({
        "id": d.task1_id,
    });
    let expected = value!({
        "taskDelete": {
            "id": d.task1_id,
        },
    });
    exec_assert(&d.schema, q, Some(v), &expected).await;

    d.tmp.drop().await
}

// OrgHandler returns org1 filter; task1 belongs to org1 -> delete succeeds.
#[tokio::test]
async fn delete_filter_match() -> Res<()> {
    let pol = row_policy("taskDelete".to_owned(), "any".to_owned());
    let cfg = AuthzConfig {
        handlers: Arc::new(OrgHandler),
        ..Default::default()
    };
    let d = row_crud_setup(pol, cfg).await?;

    let q = "
    mutation($id: ID!) {
        taskDelete(id: $id, permanent: true) {
            id
        }
    }
    ";
    let v = value!({
        "id": d.task1_id,
    });
    let expected = value!({
        "taskDelete": {
            "id": d.task1_id,
        },
    });
    exec_assert(&d.schema, q, Some(v), &expected).await;

    d.tmp.drop().await
}

// OrgHandler returns org1 filter; task2 belongs to org2 -> Unauthorized.
#[tokio::test]
async fn delete_filter_no_match() -> Res<()> {
    let pol = row_policy("taskDelete".to_owned(), "any".to_owned());
    let cfg = AuthzConfig {
        handlers: Arc::new(OrgHandler),
        ..Default::default()
    };
    let d = row_crud_setup(pol, cfg).await?;

    let q = "
    mutation($id: ID!) {
        taskDelete(id: $id, permanent: true) {
            id
        }
    }
    ";
    let v = value!({
        "id": d.task2_id,
    });
    exec_assert_err(&d.schema, q, Some(v), &AuthzErr::Unauthorized).await;

    d.tmp.drop().await
}

// unauthorized_err configured as Db404 -> filter mismatch looks like a missing row.
#[tokio::test]
async fn delete_unauthorized_err_as_db404() -> Res<()> {
    let pol = row_policy("taskDelete".to_owned(), "any".to_owned());
    let cfg = AuthzConfig {
        handlers: Arc::new(OrgHandler),
        unauthorized_err: CoreDbErr::Db404.into(),
        ..Default::default()
    };
    let d = row_crud_setup(pol, cfg).await?;

    let q = "
    mutation($id: ID!) {
        taskDelete(id: $id, permanent: true) {
            id
        }
    }
    ";
    let v = value!({
        "id": d.task2_id,
    });
    exec_assert_err(&d.schema, q, Some(v), &CoreDbErr::Db404).await;

    d.tmp.drop().await
}

// ---------------------------------------------------------------------------
// update tests
// ---------------------------------------------------------------------------

// No row_policy entry -> update succeeds without filter.
// gql_update returns G::from_id so only id is populated in the response.
#[tokio::test]
async fn update_no_policy() -> Res<()> {
    let d = row_crud_setup(RowPolicy::default(), AuthzConfig::default()).await?;

    let q = "
    mutation($id: ID!) {
        taskUpdate(id: $id, data: { title: \"Updated\" }) {
            id
        }
    }
    ";
    let v = value!({
        "id": d.task1_id,
    });
    let expected = value!({
        "taskUpdate": {
            "id": d.task1_id,
        },
    });
    exec_assert(&d.schema, q, Some(v), &expected).await;

    d.tmp.drop().await
}

// OrgHandler returns org1 filter; task1 belongs to org1 -> update succeeds.
#[tokio::test]
async fn update_filter_match() -> Res<()> {
    let pol = row_policy("taskUpdate".to_owned(), "any".to_owned());
    let cfg = AuthzConfig {
        handlers: Arc::new(OrgHandler),
        ..Default::default()
    };
    let d = row_crud_setup(pol, cfg).await?;

    let q = "
    mutation($id: ID!) {
        taskUpdate(id: $id, data: { title: \"Updated\" }) {
            id
        }
    }
    ";
    let v = value!({
        "id": d.task1_id,
    });
    let expected = value!({
        "taskUpdate": {
            "id": d.task1_id,
        },
    });
    exec_assert(&d.schema, q, Some(v), &expected).await;

    d.tmp.drop().await
}

// OrgHandler returns org1 filter; task2 belongs to org2 -> Unauthorized.
#[tokio::test]
async fn update_filter_no_match() -> Res<()> {
    let pol = row_policy("taskUpdate".to_owned(), "any".to_owned());
    let cfg = AuthzConfig {
        handlers: Arc::new(OrgHandler),
        ..Default::default()
    };
    let d = row_crud_setup(pol, cfg).await?;

    let q = "
    mutation($id: ID!) {
        taskUpdate(id: $id, data: { title: \"Updated\" }) {
            id
        }
    }
    ";
    let v = value!({
        "id": d.task2_id,
    });
    exec_assert_err(&d.schema, q, Some(v), &AuthzErr::Unauthorized).await;

    d.tmp.drop().await
}
