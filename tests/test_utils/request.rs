use super::prelude::*;

pub fn request(q: &str, v: Value) -> Request {
    Request::new(q).variables(Variables::from_value(v))
}
