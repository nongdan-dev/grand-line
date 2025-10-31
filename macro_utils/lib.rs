mod attr;
mod macros;
pub use attr::*;

#[allow(unused_imports)]
mod prelude {
    pub use crate::*;
    use_common_macro_utils!();
    use_common_std!();
}
