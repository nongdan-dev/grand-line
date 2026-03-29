#[cfg(feature = "test_utils")]
mod test_utils;

pub mod export {
    pub use _core::export::*;

    #[cfg(feature = "http")]
    pub use _http::export::*;
    #[cfg(feature = "tracing")]
    pub use _tracing::export::*;
}

pub mod reexport {
    pub use _core::reexport::*;

    #[cfg(feature = "http")]
    pub use _http::reexport::*;
    #[cfg(feature = "tracing")]
    pub use _tracing::reexport::*;
}

#[allow(ambiguous_glob_reexports, dead_code, unused_imports)]
pub mod prelude {
    pub use {_core::prelude::*, maplit::*};

    #[cfg(feature = "test_utils")]
    pub use crate::test_utils::prelude::*;
    #[cfg(feature = "http")]
    pub use _http::prelude::*;
    #[cfg(feature = "tracing")]
    pub use _tracing::prelude::*;
}

#[cfg(not(any(feature = "postgres", feature = "mysql", feature = "sqlite")))]
compile_error!("should enable one of features: postgres, mysql, sqlite");

#[cfg(all(feature = "http", not(any(feature = "axum"))))]
compile_error!("should enable one of features: axum");
