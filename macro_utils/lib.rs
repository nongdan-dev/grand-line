mod macros;

mod attr;
mod attr_debug;
mod attr_parse;
pub use attr::*;
pub use attr_debug::*;
pub use attr_parse::*;

#[allow(unused_imports)]
mod prelude {
    pub use crate::*;

    use_common_macro_utils!();
    use_common_std!();
}
