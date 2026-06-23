#![allow(ambiguous_glob_reexports, dead_code, unused_imports)]

use axum::http::HeaderMap;
pub use grand_line::prelude::*;

#[path = "../_fixtures/user.rs"]
mod user;
pub use user::*;

#[path = "../_fixtures/org.rs"]
mod org;
pub use org::*;

#[query(authz(realm = "org"))]
fn org_primitive() -> i64 {
    0
}

#[query(authz(realm = "org"))]
fn org() -> OrgGql {
    let org_id = ctx.authz().await?;
    Org::find()
        .exclude_deleted()
        .filter_by_id(&org_id)
        .gql_select(ctx)?
        .one_or_404(tx)
        .await?
}

#[query(authz(realm = "system", skip_org))]
fn system_primitive() -> i64 {
    0
}

#[query(authz(realm = "system", skip_org))]
fn system(org_id: String) -> OrgGql {
    Org::find()
        .exclude_deleted()
        .filter_by_id(&org_id)
        .gql_select(ctx)?
        .one_or_404(tx)
        .await?
}

#[derive(Default, MergedObject)]
pub struct Query(OrgPrimitiveQuery, OrgQuery, SystemPrimitiveQuery, SystemQuery);

pub struct Prepare {
    pub tmp: TmpDb,
    pub s: SchemaBuilder<Query, EmptyMutation, EmptySubscription>,
    pub h: HeaderMap,
    pub user_id1: String,
    pub user_id2: String,
    pub token1: String,
    pub token2: String,
    pub org_id1: String,
    pub org_id2: String,
    pub role_id1: String,
    pub role_id1_system: String,
    pub role_id2: String,
}

pub async fn prepare_with_col_wildcard() -> Res<Prepare> {
    prepare_with_col_policy(col_policy_wildcard()).await
}

pub async fn prepare_with_col_policy(org1_admin: ColPolicy) -> Res<Prepare> {
    prepare_with_policy(org1_admin, RowPolicy::default()).await
}

pub async fn prepare_with_policy(org1_admin: ColPolicy, org1_row: RowPolicy) -> Res<Prepare> {
    let org_impl = authz_org_impl::<Org>();

    let tmp = tmp_db!(User, LoginSession, Org, Role, UserInRole);
    let s = schema_q::<Query>(&tmp.db).data(org_impl);

    let h = init_common_headers();

    let u1 = am_create!(User {
        email: "olivia@example.com",
        password_hashed: rand_utils::password_hash("123123")?,
    })
    .exec_without_ctx(&tmp.db)
    .await?;
    let u2 = am_create!(User {
        email: "peter@example.com",
        password_hashed: rand_utils::password_hash("123123")?,
    })
    .exec_without_ctx(&tmp.db)
    .await?;

    let ua = Context::get_ua_raw(Context::get_headers_raw(&h))?;

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

    let secret2 = rand_utils::secret();
    let ls2 = am_create!(LoginSession {
        user_id: u2.id.clone(),
        secret_hashed: rand_utils::secret_hash(&secret2),
        ip: "127.0.0.1",
        ua: ua.to_json()?,
    })
    .exec_without_ctx(&tmp.db)
    .await?;
    let token2 = rand_utils::qs_token(&ls2.id, &secret2)?;

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

    let r1 = am_create!(Role {
        name: "Org Admin",
        realm: "org",
        col_policy: org1_admin.to_json()?,
        row_policy: org1_row.to_json()?,
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

    let r2 = am_create!(Role {
        name: "Org Admin",
        realm: "org",
        col_policy: col_policy_wildcard().to_json()?,
        row_policy: RowPolicy::default().to_json()?,
        org_id: Some(o2.id.clone()),
    })
    .exec_without_ctx(&tmp.db)
    .await?;
    am_create!(UserInRole {
        user_id: u2.id.clone(),
        role_id: r2.id.clone(),
        org_id: Some(o2.id.clone()),
    })
    .exec_without_ctx(&tmp.db)
    .await?;

    let r3 = am_create!(Role {
        name: "System Admin",
        realm: "system",
        col_policy: col_policy_wildcard().to_json()?,
        row_policy: RowPolicy::default().to_json()?,
    })
    .exec_without_ctx(&tmp.db)
    .await?;
    am_create!(UserInRole {
        user_id: u1.id.clone(),
        role_id: r3.id.clone(),
    })
    .exec_without_ctx(&tmp.db)
    .await?;

    Ok(Prepare {
        tmp,
        s,
        h,
        user_id1: u1.id,
        user_id2: u2.id,
        token1,
        token2,
        org_id1: o1.id,
        org_id2: o2.id,
        role_id1: r1.id,
        role_id1_system: r3.id,
        role_id2: r2.id,
    })
}

pub const fn col_policy_field(children: ColPolicyFields) -> ColPolicyField {
    ColPolicyField {
        allow: true,
        children: Some(children),
    }
}
pub const fn col_policy_field_no_children() -> ColPolicyField {
    ColPolicyField {
        allow: true,
        children: None,
    }
}

pub fn col_policy_fields(k: String, children: ColPolicyFields) -> ColPolicyFields {
    hashmap! {
        k => col_policy_field(children),
    }
}
pub fn col_policy_fields_no_children(k: String) -> ColPolicyFields {
    hashmap! {
        k => col_policy_field_no_children(),
    }
}

pub fn col_policy_fields_wildcard() -> ColPolicyFields {
    col_policy_fields_no_children("*".to_owned())
}
pub fn col_policy_fields_wildcard_nested() -> ColPolicyFields {
    col_policy_fields_no_children("**".to_owned())
}

pub const fn col_policy_operation(inputs: ColPolicyField, output: ColPolicyField) -> ColPolicyOperation {
    ColPolicyOperation {
        inputs,
        output,
    }
}
pub fn col_policy(k: String, inputs: ColPolicyField, output: ColPolicyField) -> ColPolicy {
    hashmap! {
        k => col_policy_operation(inputs, output),
    }
}

pub fn col_policy_wildcard() -> ColPolicy {
    let children = col_policy_fields_wildcard_nested();
    let field = col_policy_field(children);
    col_policy("*".to_owned(), field.clone(), field)
}

pub fn col_policy_org_name() -> ColPolicy {
    let inputs = col_policy_field(col_policy_fields_wildcard_nested());
    let output = col_policy_field(col_policy_fields_no_children("name".to_owned()));
    col_policy("org".to_owned(), inputs, output)
}

pub fn row_policy(k: String, script: String) -> RowPolicy {
    hashmap! {
        k => RowPolicyField { script }
    }
}
