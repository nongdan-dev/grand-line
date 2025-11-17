use super::prelude::*;
use axum::http::HeaderMap;

pub trait HttpAxumContext {
    fn get_headers_raw(h: &HeaderMap) -> Option<HashMap<String, Vec<String>>> {
        let mut m = HashMap::<String, Vec<String>>::new();
        for (k, v) in h.iter() {
            let k = k.as_str().to_owned();
            let v = v.to_str().unwrap_or("").to_owned();
            m.entry(k).or_default().push(v);
        }
        Some(m)
    }
    fn get_headers(&self) -> Option<HashMap<String, Vec<String>>>;
}

impl HttpAxumContext for Context<'_> {
    fn get_headers(&self) -> Option<HashMap<String, Vec<String>>> {
        let h = self.data_opt::<HeaderMap>()?;
        Self::get_headers_raw(h)
    }
}
