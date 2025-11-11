use super::prelude::*;
use axum::http::HeaderMap;
use cookie::{
    Cookie,
    // SameSite::Strict,
    time::{Duration, OffsetDateTime},
};
use std::{net::IpAddr, str::FromStr};

const REAL_IP: &str = "X-Real-IP";
const FORWARDED_FOR: &str = "X-Forwarded-For";
const SOCKET_ADDR: &str = "X-Socket-Addr";
const USER_AGENT: &str = "User-Agent";
const COOKIE: &str = "Cookie";
const SET_COOKIE: &str = "Set-Cookie";
const AUTHORIZATION: &str = "Authorization";
const BEARER: &str = "Bearer ";

pub trait GrandLineHttpContext {
    fn get_header(&self, k: &str) -> Res<String>;
    fn get_ip(&self) -> Res<String>;
    fn get_ua(&self) -> Res<String>;
    fn get_authorization_token(&self) -> Res<String>;
    fn get_cookies(&self) -> Res<HashMap<String, String>>;
    fn get_cookie(&self, k: &str) -> Res<Option<String>>;
    fn set_cookie(&self, k: &str, v: &str, expires: i64);
}

impl GrandLineHttpContext for Context<'_> {
    fn get_header(&self, k: &str) -> Res<String> {
        let req_headers = self
            .data_opt::<HeaderMap>()
            .ok_or(MyErr::CtxReqHeaders404)?;
        let v = req_headers
            .get(k)
            .map(|v| v.to_str().ok().map(|v| v.to_string()))
            .unwrap_or_default()
            .unwrap_or_default();
        Ok(v)
    }

    fn get_ip(&self) -> Res<String> {
        let mut v = self.get_header(REAL_IP)?;
        if v.is_empty() {
            v = self.get_header(FORWARDED_FOR)?;
        }
        if v.is_empty() {
            v = self.get_header(SOCKET_ADDR)?;
        }
        let ip = v.split(',').next().unwrap_or_default().trim().to_string();
        if IpAddr::from_str(&ip).is_err() {
            Err(MyErr::CtxReqIp404)?;
        }
        Ok(ip)
    }

    fn get_ua(&self) -> Res<String> {
        let ua = self.get_header(USER_AGENT)?.trim().to_string();
        if ua.is_empty() {
            Err(MyErr::CtxReqUa404)?;
        }
        Ok(ua)
    }

    fn get_authorization_token(&self) -> Res<String> {
        let v = self.get_header(AUTHORIZATION)?.replace(BEARER, "");
        Ok(v)
    }

    fn get_cookies(&self) -> Res<HashMap<String, String>> {
        let h = self.get_header(COOKIE)?;
        let mut m = HashMap::new();
        for c in h.split(';') {
            if let Ok(kv) = Cookie::parse(c) {
                m.insert(kv.name().to_string(), kv.value().to_string());
            }
        }
        Ok(m)
    }

    fn get_cookie(&self, k: &str) -> Res<Option<String>> {
        let v = self.get_cookies()?.get(k).cloned();
        Ok(v)
    }

    fn set_cookie(&self, k: &str, v: &str, expires: i64) {
        let v = Cookie::build(Cookie::new(k, v))
            .http_only(true)
            // .secure(true)
            // .same_site(Strict)
            .max_age(Duration::seconds(expires / 1000))
            .expires(OffsetDateTime::now_utc() + Duration::milliseconds(expires))
            .build()
            .to_string();
        self.append_http_header(SET_COOKIE, &v);
    }
}
