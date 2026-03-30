mod b64;
mod eq;
mod err;
mod otp;
mod password;
mod qs;
mod secret;

pub mod export {
    pub use crate::err::MyErr as AuthUtilsErr;
    pub mod rand_utils {
        pub use crate::{b64::*, eq::*, otp::*, password::*, qs::*, secret::*};
    }
}

pub mod reexport {
    pub use {argon2, base64, hmac, rand, rand_core, serde_qs, sha2, subtle, zxcvbn};
}

#[allow(ambiguous_glob_reexports, dead_code, unused_imports)]
pub mod prelude {
    pub use crate::{export::*, reexport::*};
    pub(crate) use {
        crate::export::{AuthUtilsErr as MyErr, rand_utils::*},
        _core::prelude::*,
    };
}
