mod err;
mod password;
mod qs;
mod rand;

pub mod export {
    pub use crate::err::MyErr as AuthUtilsErr;
    pub mod auth_utils {
        pub use crate::{password::*, qs::*, rand::*};
    }
}

pub mod reexport {
    pub use {argon2, base64, hmac, rand, rand_core, serde_qs, sha2, subtle, zxcvbn};
}

#[allow(ambiguous_glob_reexports, dead_code, unused_imports)]
pub mod prelude {
    pub use crate::{export::*, reexport::*};
    pub(crate) use {
        crate::export::{AuthUtilsErr as MyErr, auth_utils::*},
        _core::prelude::*,
    };
}
