use base64::{Engine, engine::general_purpose};
use rand::{Rng, RngCore, rng};

pub fn secret_256bit() -> String {
    let mut secret = [0u8; 32];
    rng().fill_bytes(&mut secret);
    general_purpose::URL_SAFE_NO_PAD.encode(secret)
}

pub fn secret_otp_6digits() -> String {
    let n: u32 = rng().random_range(0..=999_999);
    format!("{:06}", n)
}
