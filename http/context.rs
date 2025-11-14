use super::prelude::*;
use axum::http::HeaderMap;
use cookie::{
    Cookie,
    // SameSite::Strict,
    time::{Duration, OffsetDateTime},
};
use std::{net::IpAddr, str::FromStr};

const REAL_IP: &str = "x-real-ip";
const FORWARDED_FOR: &str = "x-forwarded-for";
const SOCKET_ADDR: &str = "x-socket-addr";
const USER_AGENT: &str = "user-agent";
const SEC_CH_UA: &str = "sec-ch-ua";
const COOKIE: &str = "cookie";
const SET_COOKIE: &str = "set-cookie";
const AUTHORIZATION: &str = "authorization";
const BEARER: &str = "Bearer ";

pub trait HttpContext {
    fn get_header(&self, k: &str) -> Res<String>;
    fn get_ip(&self) -> Res<String>;
    fn get_ua(&self) -> Res<HashMap<String, String>>;
    fn get_authorization_token(&self) -> Res<String>;
    fn get_cookies(&self) -> Res<HashMap<String, String>>;
    fn get_cookie(&self, k: &str) -> Res<Option<String>>;
    fn set_cookie(&self, k: &str, v: &str, expires: i64);
}

impl HttpContext for Context<'_> {
    fn get_header(&self, k: &str) -> Res<String> {
        let req_headers = self
            .data_opt::<HeaderMap>()
            .ok_or(MyErr::CtxReqHeaders404)?;
        let v = req_headers
            .get(k)
            .map(|v| v.to_str().ok().map(|v| v.to_owned()))
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
        let ip = v.split(',').next().unwrap_or_default().trim().to_owned();
        if IpAddr::from_str(&ip).is_err() {
            Err(MyErr::CtxReqIp404)?;
        }
        Ok(ip)
    }

    fn get_ua(&self) -> Res<HashMap<String, String>> {
        if self.get_header(USER_AGENT)?.is_empty() {
            Err(MyErr::CtxReqUa404)?;
        }
        let h = self
            .data_opt::<HeaderMap>()
            .ok_or(MyErr::CtxReqHeaders404)?;
        Ok(get_ua(h))
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
                m.insert(kv.name().to_owned(), kv.value().to_owned());
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

pub fn get_ua(h: &HeaderMap) -> HashMap<String, String> {
    h.iter()
        .filter_map(|(k, v)| {
            let k = k.as_str();
            if k.starts_with(SEC_CH_UA) || k == USER_AGENT {
                Some((k.to_owned(), v.to_str().unwrap_or_default().to_owned()))
            } else {
                None
            }
        })
        .collect()
}
