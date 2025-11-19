use crate::prelude::*;
use axum::http::{HeaderMap, HeaderValue};

pub fn init_common_headers() -> HeaderMap {
    let mut h = HeaderMap::default();
    h.insert(H_REAL_IP, h_static("127.0.0.1"));
    h.insert(H_UA, h_static(UA));
    h.insert(H_UA_SEC_CH, h_static(UA_SEC_CH));
    h
}

pub fn h_static(v: &'static str) -> HeaderValue {
    HeaderValue::from_static(v)
}
pub fn h_str(v: &str) -> HeaderValue {
    HeaderValue::from_str(v).unwrap_or_else(|_| h_static(""))
}

pub fn h_bearer(token: &str) -> HeaderValue {
    let v = format!("{BEARER}{token}");
    h_str(&v)
}
