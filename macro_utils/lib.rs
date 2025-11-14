mod macros;

mod attr;
pub use attr::*;

pub use heck;
pub use maplit;
pub use proc_macro2;
pub use quote;
pub use serde;
pub use strum;
pub use strum_macros;
pub use syn;

#[allow(unused_imports, dead_code)]
mod prelude {
    pub use crate::*;
    use_common_macro_utils!();
    use_common_std!();
}
