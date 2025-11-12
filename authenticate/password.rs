use super::prelude::*;
use argon2::{
    Argon2, PasswordHasher, PasswordVerifier,
    password_hash::{PasswordHash, SaltString},
};
use rand::{RngCore, rng};

pub fn password_hash(password: &str) -> Res<String> {
    let mut b = [0u8; 16];
    rng().fill_bytes(&mut b);
    let salt = SaltString::encode_b64(&b).map_err(MyErr::from)?;
    let password_hashed = Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map_err(MyErr::from)?
        .to_string();
    Ok(password_hashed)
}

pub fn password_compare(password_hashed: &str, password: &str) -> bool {
    match PasswordHash::new(password_hashed) {
        Ok(v) => Argon2::default()
            .verify_password(password.as_bytes(), &v)
            .is_ok(),
        _ => false,
    }
}
