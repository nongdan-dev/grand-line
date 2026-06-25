#![allow(ambiguous_glob_reexports, dead_code, unused_imports)]

#[path = "./setup.rs"]
mod setup;
pub use setup::*;
#[path = "./row_handlers.rs"]
mod row_handlers;
pub use row_handlers::*;

// ---------------------------------------------------------------------------
// CRUD resolvers with authz row filtering
// authz_row defaults to true in tests via resolver_authz_row feature flag.
// ---------------------------------------------------------------------------

#[search(Task, authz(realm = "org"))]
fn task_search() {
    (None, None)
}

#[count(Task, authz(realm = "org"))]
fn task_count() {
    None
}

#[detail(Task, authz(realm = "org"))]
fn task_detail() {
}

#[delete(Task, authz(realm = "org"))]
fn task_delete() {
}

#[gql_input]
pub struct TaskUpdate {
    pub title: String,
}

#[update(Task, authz(realm = "org"))]
fn task_update() {
    am_update!(Task {
        id: id.clone(),
        title: data.title,
    })
}

#[derive(Default, MergedObject)]
pub struct CrudQ(TaskSearchQuery, TaskCountQuery, TaskDetailQuery);
#[derive(Default, MergedObject)]
pub struct CrudM(TaskDeleteMutation, TaskUpdateMutation);

// ---------------------------------------------------------------------------
// Setup
// ---------------------------------------------------------------------------

pub struct RowCrudSetup {
    pub tmp: TmpDb,
    pub schema: GraphQLSchema<CrudQ, CrudM, EmptySubscription>,
    // task1: org1, task2: org2
    pub task1_id: String,
    pub task2_id: String,
}

pub async fn row_crud_setup(row_pol: RowPolicy, cfg: AuthzConfig) -> Res<RowCrudSetup> {
    let org_impl = authz_org_impl::<Org>();
    let tmp = tmp_db!(User, LoginSession, Org, Role, UserInRole, Task);
    let s = schema_qm::<CrudQ, CrudM>(&tmp.db).data(org_impl).data(cfg);

    let h = init_common_headers();
    let ua = Context::get_ua_raw(Context::get_headers_raw(&h))?;

    let u1 = am_create!(User {
        email: "alice@example.com",
        password_hashed: rand_utils::password_hash("pw")?,
    })
    .exec_without_ctx(&tmp.db)
    .await?;

    let secret1 = rand_utils::secret();
    let ls1 = am_create!(LoginSession {
        user_id: u1.id.clone(),
        secret_hashed: rand_utils::secret_hash(&secret1),
        ip: "127.0.0.1",
        ua: ua.to_json()?,
    })
    .exec_without_ctx(&tmp.db)
    .await?;
    let token1 = rand_utils::qs_token(&ls1.id, &secret1)?;

    let o1 = am_create!(Org {
        name: "Alpha"
    })
    .exec_without_ctx(&tmp.db)
    .await?;
    let o2 = am_create!(Org {
        name: "Beta"
    })
    .exec_without_ctx(&tmp.db)
    .await?;

    let r1 = am_create!(Role {
        name: "Admin",
        realm: "org",
        col_policy: col_policy_wildcard().to_json()?,
        row_policy: row_pol.to_json()?,
        org_id: Some(o1.id.clone()),
    })
    .exec_without_ctx(&tmp.db)
    .await?;
    am_create!(UserInRole {
        user_id: u1.id.clone(),
        role_id: r1.id.clone(),
        org_id: Some(o1.id.clone()),
    })
    .exec_without_ctx(&tmp.db)
    .await?;

    // task1: org1, task2: org2
    let t1 = am_create!(Task {
        title: "Task1",
        assignee_id: u1.id.clone(),
        org_id: o1.id.clone(),
    })
    .exec_without_ctx(&tmp.db)
    .await?;
    let t2 = am_create!(Task {
        title: "Task2",
        assignee_id: u1.id.clone(),
        org_id: o2.id.clone(),
    })
    .exec_without_ctx(&tmp.db)
    .await?;

    let mut headers = h;
    headers.append(H_ORG_ID, h_str(&o1.id));
    headers.insert(H_AUTHORIZATION, h_bearer(&token1));
    headers.insert(H_ROLE_ID, h_str(&r1.id));

    Ok(RowCrudSetup {
        schema: s.data(headers).finish(),
        tmp,
        task1_id: t1.id,
        task2_id: t2.id,
    })
}
