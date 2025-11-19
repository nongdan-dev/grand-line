#![allow(ambiguous_glob_reexports, dead_code, unused_imports)]

use axum::http::{HeaderMap, HeaderValue};
pub use grand_line::prelude::*;

#[query(authz)]
fn org_primitive() -> i64 {
    0
}

#[query(authz)]
fn org() -> OrgGql {
    let org_id = ctx.authz().await?;
    Org::find()
        .exclude_deleted()
        .filter_by_id(&org_id)
        .gql_select(ctx)?
        .one_or_404(tx)
        .await?
}

#[query(authz(user, key = "system"))]
fn system_primitive() -> i64 {
    0
}

#[query(authz(user, key = "system"))]
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

    let mut h = HeaderMap::default();
    h.insert("x-real-ip", h_static("127.0.0.1"));
    h.insert("user-agent", h_static(UA));
    h.insert("sec-ch-ua", h_static(UA_SEC_CH));

    let u1 = am_create!(User {
        email: "olivia@example.com",
        password_hashed: rand_utils::password_hash("123123")?,
    })
    .insert(&tmp.db)
    .await?;
    let u2 = am_create!(User {
        email: "olivia@example.com",
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

    let mut allow_all_fields = HashMap::<String, OperationFieldPolicy>::new();
    allow_all_fields.insert(
        "**".to_owned(),
        OperationFieldPolicy {
            allow: true,
            ..Default::default()
        },
    );

    let mut allow_all_operations = HashMap::<String, OperationPolicy>::new();
    allow_all_operations.insert(
        "*".to_owned(),
        OperationPolicy {
            inputs: OperationFieldPolicy {
                allow: true,
                children: allow_all_fields.clone(),
            },
            output: OperationFieldPolicy {
                allow: true,
                children: allow_all_fields,
            },
        },
    );

    let r1 = am_create!(Role {
        name: "Org Admin",
        operations: allow_all_operations.to_json()?,
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
        operations: allow_all_operations.to_json()?,
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
        key: Some("system".to_owned()),
        operations: allow_all_operations.to_json()?,
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

pub fn h_static(v: &'static str) -> HeaderValue {
    HeaderValue::from_static(v)
}
pub fn h_str(v: &str) -> HeaderValue {
    HeaderValue::from_str(v).unwrap_or_else(|_| h_static(""))
}

pub struct MockAuthHandlers;
#[async_trait]
impl AuthHandlers for MockAuthHandlers {
    async fn otp(&self, _ctx: &Context<'_>) -> Res<String> {
        Ok("999999".to_owned())
    }
}

const UA: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/142.0.0.0 Safari/537.36";
const UA_SEC_CH: &str = r#""Chromium";v="142", "Google Chrome";v="142", "Not_A Brand";v="99""#;
