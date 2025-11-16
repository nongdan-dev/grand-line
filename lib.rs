#[cfg(feature = "test_utils")]
mod test_utils;

pub mod export {
    pub use _core::export::*;

    #[cfg(feature = "auth")]
    pub use _auth::export::*;
    #[cfg(feature = "auth_utils")]
    pub use _auth_utils::export::*;
    #[cfg(feature = "http")]
    pub use _http::export::*;
    #[cfg(feature = "tracing")]
    pub use _tracing::export::*;
}

pub mod reexport {
    pub use _core::reexport::*;

    #[cfg(feature = "auth")]
    pub use _auth::reexport::*;
    #[cfg(feature = "auth_utils")]
    pub use _auth_utils::reexport::*;
    #[cfg(feature = "http")]
    pub use _http::reexport::*;
    #[cfg(feature = "tracing")]
    pub use _tracing::reexport::*;
}

#[allow(ambiguous_glob_reexports, dead_code, unused_imports)]
pub mod prelude {
    pub use _core::prelude::*;

    #[cfg(feature = "test_utils")]
    pub use crate::test_utils::prelude::*;
    #[cfg(feature = "auth")]
    pub use _auth::prelude::*;
    #[cfg(feature = "auth_utils")]
    pub use _auth_utils::prelude::*;
    #[cfg(feature = "http")]
    pub use _http::prelude::*;
    #[cfg(feature = "tracing")]
    pub use _tracing::prelude::*;
}
