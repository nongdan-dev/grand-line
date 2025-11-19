pub mod consts;
mod context;
mod err;

pub mod export {
    pub use crate::{context::*, err::MyErr as HttpErr};

    #[cfg(feature = "axum")]
    pub use _http_axum::export::*;
}

pub mod reexport {
    pub use cookie;

    #[cfg(feature = "axum")]
    pub use _http_axum::reexport::*;
}

#[allow(ambiguous_glob_reexports, dead_code, unused_imports)]
pub mod prelude {
    pub use crate::{export::*, reexport::*};
    pub(crate) use {
        crate::{consts::*, err::MyErr},
        _core::prelude::*,
    };

    #[cfg(feature = "axum")]
    pub use _http_axum::prelude::*;
}
