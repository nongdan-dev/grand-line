use crate::prelude::*;
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD as B64};
use hmac::{Hmac, Mac};
use rand::{Rng, rng};
use sha2::Sha256;

pub fn otp() -> String {
    let otp = rng().random_range(0..=999_999);
    format!("{otp:06}")
}

pub fn otp_hash(otp: &str) -> Res<(String, String)> {
    let salt = b64_random(8);
    let otp_hashed = otp_hash_with_salt(&salt, otp)?;
    Ok((salt, otp_hashed))
}

pub fn otp_eq(salt: &str, otp_hashed: &str, otp: &str) -> Res<bool> {
    let otp_hashed2 = otp_hash_with_salt(salt, otp)?;
    let r = constant_time_eq(otp_hashed, &otp_hashed2);
    Ok(r)
}

pub(crate) fn otp_hash_with_salt(salt: &str, otp: &str) -> Res<String> {
    let mut mac = Hmac::<Sha256>::new_from_slice(salt.as_bytes()).map_err(|e| MyErr::HmacErr {
        inner: e.to_string(),
    })?;
    mac.update(otp.as_bytes());
    let b = mac.finalize().into_bytes();
    let secret = B64.encode(b);
    Ok(secret)
}
