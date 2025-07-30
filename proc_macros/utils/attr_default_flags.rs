use crate::prelude::*;

attr_default_flag!(no_created_at);
attr_default_flag!(no_updated_at);
attr_default_flag!(no_deleted_at);
attr_default_flag!(no_by_id);

attr_default_flag!(resolver_inputs);
attr_default_flag!(resolver_output);
attr_default_flag!(no_tx);
attr_default_flag!(no_ctx);

pub fn default_limit_default() -> u64 {
    let v = 10;
    #[cfg(feature = "limit_x2")]
    let v = 20;
    #[cfg(feature = "limit_x5")]
    let v = 50;
    #[cfg(feature = "limit_x10")]
    let v = 100;
    v
}
pub fn default_limit_max() -> u64 {
    10 * default_limit_default()
}
