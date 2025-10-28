#![allow(unused_imports, ambiguous_glob_reexports)]

mod macros;
mod mods;
pub use mods::*;

mod prelude {
    pub use crate::*;
    use_common_macro_utils!();
    use_common_std!();
}
