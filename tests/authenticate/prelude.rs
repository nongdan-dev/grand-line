#![allow(dead_code)]

use axum::http::{HeaderMap, HeaderValue};
pub use grand_line::prelude::*;

pub struct Prepare {
    pub tmp: TmpDb,
    pub s: SchemaBuilder<AuthenticateMergedQuery, AuthenticateMergedMutation, EmptySubscription>,
    pub h: HeaderMap,
    pub user_id: String,
    pub token: String,
}

pub async fn prepare() -> Res<Prepare> {
    let tmp = tmp_db!(User, AuthOtp, LoginSession);
    let s = schema_qm::<AuthenticateMergedQuery, AuthenticateMergedMutation>(&tmp.db);

    let mut h = HeaderMap::default();
    h.insert("X-Real-IP", h_static("127.0.0.1"));
    h.insert("User-Agent", h_static("test user agent"));

    let u = db_create!(
        &tmp.db,
        User {
            email: "olivia@example.com",
            password_hashed: password_hash("123123")?,
        }
    );
    let ls = db_create!(
        &tmp.db,
        LoginSession {
            user_id: u.id.clone(),
            ip: "127.0.0.1",
            ua: "test user agent",
        }
    );
    let token = qs_token(&ls.id, &ls.secret)?;

    Ok(Prepare {
        tmp,
        s,
        h,
        user_id: u.id,
        token,
    })
}

pub fn h_static(v: &'static str) -> HeaderValue {
    HeaderValue::from_static(v)
}
pub fn h_str(v: &str) -> HeaderValue {
    HeaderValue::from_str(v).unwrap_or_else(|_| h_static(""))
}
