use crate::prelude::*;
use axum::http::HeaderMap;
use std::{net::IpAddr, str::FromStr};

pub trait ContextHttpImpl {
    fn get_header(&self, k: &str) -> Res<String>;
    fn get_ip(&self) -> Res<String>;
    fn get_ua(&self) -> Res<String>;
}

impl ContextHttpImpl for Context<'_> {
    fn get_header(&self, k: &str) -> Res<String> {
        let req_headers = self
            .data::<HeaderMap>()
            .map_err(|e| MyErr::CtxReqHeaders404 { inner: e.message })?;
        let v = req_headers
            .get(k)
            .map(|v| v.to_str().ok().map(|v| v.to_string()))
            .unwrap_or_default()
            .unwrap_or_default();
        Ok(v)
    }

    fn get_ip(&self) -> Res<String> {
        let mut v = self.get_header("x-forwarded-for")?;
        if v.is_empty() {
            v = self.get_header("x-socket-addr")?;
        }
        let ip = v.split(',').next().unwrap_or_default().trim().to_string();
        if IpAddr::from_str(&ip).is_err() {
            err!(CtxReqIp404)?;
        }
        Ok(ip)
    }

    fn get_ua(&self) -> Res<String> {
        let ua = self.get_header("user-agent")?.trim().to_string();
        if ua.is_empty() {
            err!(CtxReqUa404)?;
        }
        Ok(ua)
    }
}
