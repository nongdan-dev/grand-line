use crate::prelude::*;
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD as B64};
use rand::{RngCore, rng};

pub fn b64_random(bytes: usize) -> String {
    let mut b = vec![0u8; bytes];
    rng().fill_bytes(&mut b);
    B64.encode(b)
}

pub fn b64_encode(s: &str) -> String {
    B64.encode(s.as_bytes())
}
