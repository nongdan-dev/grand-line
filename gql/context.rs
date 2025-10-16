use crate::prelude::*;
use axum::http::HeaderMap;
use std::{net::IpAddr, str::FromStr};

pub trait ContextX
where
    Self: GrandLineContextImpl,
{
    fn req_header(&self, k: &str) -> Res<String>;
    fn req_ip(&self) -> Res<String>;
    fn req_ua(&self) -> Res<String>;
}

impl ContextX for Context<'_> {
    fn req_header(&self, k: &str) -> Res<String> {
        let req_headers = self
            .data::<HeaderMap>()
            .map_err(|e| err_server!(CtxReqHeaders404(e.message)))?;
        let v = req_headers
            .get(k)
            .map(|v| v.to_str().ok().map(|v| v.to_string()))
            .unwrap_or_default()
            .unwrap_or_default();
        Ok(v)
    }
    fn req_ip(&self) -> Res<String> {
        let mut v = self.req_header("x-forwarded-for")?;
        if v.is_empty() {
            v = self.req_header("x-socket-addr")?;
        }
        let ip = v.split(',').next().unwrap_or_default().trim().to_string();
        if IpAddr::from_str(&ip).is_err() {
            err_client_res!(CtxReqIp404)?;
        }
        Ok(ip)
    }
    fn req_ua(&self) -> Res<String> {
        let ua = self.req_header("user-agent")?.trim().to_string();
        if ua.is_empty() {
            err_client_res!(CtxReqUa404)?;
        }
        Ok(ua)
    }
}
