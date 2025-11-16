#![allow(ambiguous_glob_reexports, dead_code, unused_imports)]

use axum::http::{HeaderMap, HeaderValue};
pub use grand_line::prelude::*;

pub struct Prepare {
    pub tmp: TmpDb,
    pub s: SchemaBuilder<AuthMergedQuery, AuthMergedMutation, EmptySubscription>,
    pub h: HeaderMap,
    pub user_id: String,
    pub token: String,
}

pub async fn prepare() -> Res<Prepare> {
    let tmp = tmp_db!(User, AuthOtp, LoginSession);
    let s = schema_qm::<AuthMergedQuery, AuthMergedMutation>(&tmp.db).data(AuthConfig {
        handlers: Arc::new(MockAuthHandlers),
        ..Default::default()
    });

    let mut h = HeaderMap::default();
    h.insert("x-real-ip", h_static("127.0.0.1"));
    h.insert("user-agent", h_static(UA));
    h.insert("sec-ch-ua", h_static(UA_SEC_CH));
    let ua = Context::get_ua_raw(Context::get_headers_raw(&h))?;

    let u = db_create!(
        &tmp.db,
        User {
            email: "olivia@example.com",
            password_hashed: auth_utils::password_hash("123123")?,
        },
    );
    let ls = db_create!(
        &tmp.db,
        LoginSession {
            user_id: u.id.clone(),
            ip: "127.0.0.1",
            ua: ua.to_json()?,
        },
    );
    let token = auth_utils::qs_token(&ls.id, &ls.secret)?;

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

pub struct MockAuthHandlers;
#[async_trait]
impl AuthHandlers for MockAuthHandlers {
    async fn otp(&self, _ctx: &Context<'_>) -> Res<String> {
        Ok("999999".to_owned())
    }
}

const UA: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/142.0.0.0 Safari/537.36";
const UA_SEC_CH: &str = r#""Chromium";v="142", "Google Chrome";v="142", "Not_A Brand";v="99""#;
