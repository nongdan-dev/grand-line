use super::prelude::*;
use axum::http::HeaderMap;

pub trait HttpAxumContext<'a>
where
    Self: CoreContext<'a>,
{
    fn axum_headers(h: &HeaderMap) -> Option<HashMap<String, Vec<String>>> {
        let mut m = HashMap::<String, Vec<String>>::new();
        for (k, v) in h {
            let k = k.as_str().to_owned();
            let v = v.to_str().unwrap_or("").to_owned();
            m.entry(k).or_default().push(v);
        }
        Some(m)
    }

    fn get_headers(&self) -> Option<HashMap<String, Vec<String>>> {
        let h = self.data_opt_impl::<HeaderMap>()?;
        Self::axum_headers(h)
    }
}

impl<'a> HttpAxumContext<'a> for Context<'a> {
}
