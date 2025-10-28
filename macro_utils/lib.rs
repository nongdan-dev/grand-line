#![allow(unused_imports, ambiguous_glob_reexports)]

mod attr;
mod macros;
pub use attr::*;

mod prelude {
    pub use crate::*;
    use_common_macro_utils!();
    use_common_std!();
}
