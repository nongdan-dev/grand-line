#![allow(ambiguous_glob_reexports, dead_code, unused_imports)]

use axum::http::HeaderMap;
pub use grand_line::prelude::*;

#[query(authz(key = "admin", org, user))]
fn org_primitive() -> i64 {
    0
}

#[query(authz(key = "admin", org, user))]
fn org() -> OrgGql {
    let org_id = ctx.authz().await?;
    Org::find()
        .exclude_deleted()
        .filter_by_id(&org_id)
        .gql_select(ctx)?
        .one_or_404(tx)
        .await?
}

#[query(authz(key = "system", user))]
fn system_primitive() -> i64 {
    0
}

#[query(authz(key = "system", user))]
fn system(org_id: String) -> OrgGql {
    Org::find()
        .exclude_deleted()
        .filter_by_id(&org_id)
        .gql_select(ctx)?
        .one_or_404(tx)
        .await?
}

#[derive(Default, MergedObject)]
pub struct Query(
    OrgPrimitiveQuery,
    OrgQuery,
    SystemPrimitiveQuery,
    SystemQuery,
);

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
}

pub async fn prepare() -> Res<Prepare> {
    let tmp = tmp_db!(User, LoginSession, Org, Role, UserInRole);
    let s = schema_q::<Query>(&tmp.db);
    let h = init_common_headers();

    let u1 = am_create!(User {
        email: "olivia@example.com",
        password_hashed: rand_utils::password_hash("123123")?,
    })
    .insert(&tmp.db)
    .await?;
    let u2 = am_create!(User {
        email: "peter@example.com",
        password_hashed: rand_utils::password_hash("123123")?,
    })
    .insert(&tmp.db)
    .await?;

    let ua = Context::get_ua_raw(Context::get_headers_raw(&h))?;
    let ls1 = am_create!(LoginSession {
        user_id: u1.id.clone(),
        ip: "127.0.0.1",
        ua: ua.to_json()?,
    })
    .insert(&tmp.db)
    .await?;
    let token1 = rand_utils::qs_token(&ls1.id, &ls1.secret)?;
    let ls2 = am_create!(LoginSession {
        user_id: u2.id.clone(),
        ip: "127.0.0.1",
        ua: ua.to_json()?,
    })
    .insert(&tmp.db)
    .await?;
    let token2 = rand_utils::qs_token(&ls2.id, &ls2.secret)?;

    let o1 = am_create!(Org { name: "Fringe" }).insert(&tmp.db).await?;
    let o2 = am_create!(Org { name: "FBI" }).insert(&tmp.db).await?;

    let r1 = am_create!(Role {
        name: "Org Admin",
        key: "admin",
        operations: operations_wildcard().to_json()?,
        org_id: Some(o1.id.clone()),
    })
    .insert(&tmp.db)
    .await?;
    am_create!(UserInRole {
        user_id: u1.id.clone(),
        role_id: r1.id.clone(),
        org_id: Some(o1.id.clone()),
    })
    .insert(&tmp.db)
    .await?;

    let r2 = am_create!(Role {
        name: "Org Admin",
        key: "admin",
        operations: operations_wildcard().to_json()?,
        org_id: Some(o2.id.clone()),
    })
    .insert(&tmp.db)
    .await?;
    am_create!(UserInRole {
        user_id: u2.id.clone(),
        role_id: r2.id.clone(),
        org_id: Some(o2.id.clone()),
    })
    .insert(&tmp.db)
    .await?;

    let r3 = am_create!(Role {
        name: "System Admin",
        key: "system",
        operations: operations_wildcard().to_json()?,
    })
    .insert(&tmp.db)
    .await?;
    am_create!(UserInRole {
        user_id: u1.id.clone(),
        role_id: r3.id.clone(),
    })
    .insert(&tmp.db)
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
    })
}

pub fn field(children: PolicyFields) -> PolicyField {
    PolicyField {
        allow: true,
        children: Some(children),
    }
}
pub fn field_no_children() -> PolicyField {
    PolicyField {
        allow: true,
        children: None,
    }
}
pub fn fields(k: String, children: PolicyFields) -> PolicyFields {
    hashmap! {
        k => field(children),
    }
}
pub fn fields_no_children(k: String) -> PolicyFields {
    hashmap! {
        k => field_no_children(),
    }
}
pub fn fields_wildcard() -> PolicyFields {
    fields_no_children("*".to_owned())
}
pub fn fields_wildcard_nested() -> PolicyFields {
    fields_no_children("**".to_owned())
}

pub fn operation(inputs: PolicyField, output: PolicyField) -> PolicyOperation {
    PolicyOperation { inputs, output }
}
pub fn operations(k: String, inputs: PolicyField, output: PolicyField) -> PolicyOperations {
    hashmap! {
        k => operation(inputs, output),
    }
}
pub fn operations_wildcard() -> PolicyOperations {
    let children = fields_wildcard_nested();
    let field = field(children);
    operations("*".to_owned(), field.clone(), field)
}
