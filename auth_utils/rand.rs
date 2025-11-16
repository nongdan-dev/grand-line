use crate::prelude::*;
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD as B64};
use hmac::{Hmac, Mac};
use rand::{Rng, RngCore, rng};
use sha2::Sha256;
use subtle::ConstantTimeEq;

pub fn secret() -> String {
    random_b64(32)
}

pub fn otp() -> String {
    let otp = rng().random_range(0..=999_999);
    format!("{:06}", otp)
}
pub fn otp_hash(otp: &str) -> Res<(String, String)> {
    let salt = random_b64(8);
    let otp_hashed = otp_hash_with_salt(&salt, otp)?;
    Ok((salt, otp_hashed))
}
pub fn otp_eq(salt: &str, otp_hashed: &str, otp: &str) -> Res<bool> {
    let otp_hashed2 = otp_hash_with_salt(salt, otp)?;
    let r = constant_time_eq(otp_hashed, &otp_hashed2);
    Ok(r)
}

pub fn otp_hash_with_salt(salt: &str, otp: &str) -> Res<String> {
    let mut mac = Hmac::<Sha256>::new_from_slice(salt.as_bytes()).map_err(|e| MyErr::HmacErr {
        inner: e.to_string(),
    })?;
    mac.update(otp.as_bytes());
    let b = mac.finalize().into_bytes();
    let secret = B64.encode(b);
    Ok(secret)
}

pub fn random_b64(bytes: usize) -> String {
    let mut b = vec![0u8; bytes];
    rng().fill_bytes(&mut b);
    B64.encode(b)
}
pub fn constant_time_eq(a: &str, b: &str) -> bool {
    a.as_bytes().ct_eq(b.as_bytes()).unwrap_u8() == 1
}
