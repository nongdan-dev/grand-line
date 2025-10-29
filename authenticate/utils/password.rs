use crate::prelude::*;
use argon2::{
    Argon2, PasswordHasher, PasswordVerifier,
    password_hash::{Error as PasswordHashErr, PasswordHash, SaltString},
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

pub fn password_compare(password: &str, password_hashed: &str) -> Res<bool> {
    match PasswordHash::new(password_hashed) {
        Ok(v) => match Argon2::default().verify_password(password.as_bytes(), &v) {
            Ok(_) => Ok(true),
            Err(PasswordHashErr::Password) => Ok(false),
            Err(e) => Err(MyErr::from(e))?,
        },
        Err(e) => Err(MyErr::from(e))?,
    }
}
