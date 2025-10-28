#![allow(unused_imports, ambiguous_glob_reexports)]

#[cfg(feature = "authenticate")]
pub use _authenticate::export::*;
#[cfg(feature = "axum")]
pub use _axum::export::*;
pub use _core::export::*;

mod reexport {
    #[cfg(feature = "axum")]
    pub use _axum::reexport::*;
    pub use _core::reexport::*;
}
#[cfg(not(feature = "no_reexport_dep"))]
pub use reexport::*;

mod alias {
    pub use _core::alias::*;
}
#[cfg(not(feature = "no_reexport_alias"))]
pub use alias::*;

pub mod prelude {
    #[cfg(feature = "authenticate")]
    pub use _authenticate::prelude::*;
    #[cfg(feature = "axum")]
    pub use _axum::prelude::*;
    pub use _core::prelude::*;
}
