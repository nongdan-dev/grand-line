mod attr;
mod macros;
pub use attr::*;

pub use {heck, maplit, proc_macro2, quote, serde, strum, strum_macros, syn};

#[allow(unused_imports, dead_code)]
mod prelude {
    pub use crate::*;
    use_common_macro_utils!();
    use_common_std!();
}
