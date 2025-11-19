use super::prelude::*;
use cookie::{
    Cookie,
    time::{Duration, OffsetDateTime},
};
use std::{net::IpAddr, str::FromStr};

pub trait HttpContext {
    fn get_ua_raw(h: Option<HashMap<String, Vec<String>>>) -> Res<HashMap<String, String>> {
        let mut m = HashMap::<String, String>::new();
        for (k, v) in h.ok_or(MyErr::CtxHeaders404)?.iter() {
            let k = k.as_str();
            if k.starts_with(H_UA_SEC_CH) || k == H_UA {
                if v.len() > 1 {
                    Err(MyErr::HeaderMultipleValues { k: k.to_owned() })?;
                }
                m.insert(k.to_owned(), v.first().cloned().unwrap_or_default());
            }
        }
        Ok(m)
    }

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
        let req_headers = self.get_headers().ok_or(MyErr::CtxHeaders404)?;
        let v = if let Some(v) = req_headers.get(k) {
            v
        } else {
            return Ok("".to_owned());
        };
        if v.len() > 1 {
            Err(MyErr::HeaderMultipleValues { k: k.to_owned() })?;
        }
        let v = v.first().cloned().unwrap_or_default();
        Ok(v)
    }

    fn get_ip(&self) -> Res<String> {
        let mut v = self.get_header(H_REAL_IP)?;
        if v.is_empty() {
            v = self.get_header(H_FORWARDED_FOR)?;
        }
        if v.is_empty() {
            v = self.get_header(H_SOCKET_ADDR)?;
        }
        let ip = v.split(',').next().unwrap_or_default().trim().to_owned();
        if IpAddr::from_str(&ip).is_err() {
            Err(MyErr::HeaderIp404)?;
        }
        Ok(ip)
    }

    fn get_ua(&self) -> Res<HashMap<String, String>> {
        if self.get_header(H_UA)?.is_empty() {
            Err(MyErr::HeaderUa404)?;
        }
        let h = self.get_headers();
        let ua = Self::get_ua_raw(h)?;
        Ok(ua)
    }

    fn get_authorization_token(&self) -> Res<String> {
        let v = self.get_header(H_AUTHORIZATION)?.replace(BEARER, "");
        Ok(v)
    }

    fn get_cookies(&self) -> Res<HashMap<String, String>> {
        let h = self.get_header(H_COOKIE)?;
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
            .secure(true)
            .max_age(Duration::seconds(expires / 1000))
            .expires(OffsetDateTime::now_utc() + Duration::milliseconds(expires))
            .build()
            .to_string();
        self.append_http_header(H_SET_COOKIE, &v);
    }
}
