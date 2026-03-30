use crate::prelude::*;
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD as B64};
use sha2::{Digest, Sha256};

pub fn secret() -> String {
    b64_random(32)
}

pub fn secret_hash(secret: &str) -> String {
    let b = Sha256::digest(secret.as_bytes());
    B64.encode(b)
}

pub fn secret_eq(secret_hashed: &str, secret: &str) -> bool {
    let secret_hashed2 = secret_hash(secret);
    constant_time_eq(secret_hashed, &secret_hashed2)
}
