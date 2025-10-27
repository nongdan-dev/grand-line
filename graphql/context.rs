use super::prelude::*;
use axum::http::HeaderMap;
use std::{net::IpAddr, str::FromStr};

pub trait ContextX
where
    Self: GrandLineContextImpl,
{
    fn try_look_ahead(&self) -> Res<SelectionField<'_>>;
    fn req_header(&self, k: &str) -> Res<String>;
    fn req_ip(&self) -> Res<String>;
    fn req_ua(&self) -> Res<String>;
}

impl ContextX for Context<'_> {
    fn try_look_ahead(&self) -> Res<SelectionField<'_>> {
        let f = self.look_ahead().selection_fields();
        if f.len() != 1 {
            err!(LookAhead)?;
        }
        Ok(f[0])
    }
    fn req_header(&self, k: &str) -> Res<String> {
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
    fn req_ip(&self) -> Res<String> {
        let mut v = self.req_header("x-forwarded-for")?;
        if v.is_empty() {
            v = self.req_header("x-socket-addr")?;
        }
        let ip = v.split(',').next().unwrap_or_default().trim().to_string();
        if IpAddr::from_str(&ip).is_err() {
            err!(CtxReqIp404)?;
        }
        Ok(ip)
    }
    fn req_ua(&self) -> Res<String> {
        let ua = self.req_header("user-agent")?.trim().to_string();
        if ua.is_empty() {
            err!(CtxReqUa404)?;
        }
        Ok(ua)
    }
}
