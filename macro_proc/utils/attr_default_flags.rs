pub const FEATURE_NO_CREATED_AT: bool = cfg!(feature = "no_created_at");
pub const FEATURE_NO_UPDATED_AT: bool = cfg!(feature = "no_updated_at");
pub const FEATURE_NO_DELETED_AT: bool = cfg!(feature = "no_deleted_at");
pub const FEATURE_NO_BY_ID: bool = cfg!(feature = "no_by_id");

pub const FEATURE_RESOLVER_INPUTS: bool = cfg!(feature = "resolver_inputs");
pub const FEATURE_RESOLVER_OUTPUT: bool = cfg!(feature = "resolver_output");
pub const FEATURE_NO_TX: bool = cfg!(feature = "no_tx");
pub const FEATURE_NO_CTX: bool = cfg!(feature = "no_ctx");
pub const FEATURE_NO_INCLUDE_DELETED: bool = cfg!(feature = "no_include_deleted");
pub const FEATURE_NO_PERMANENT_DELETE: bool = cfg!(feature = "no_permanent_delete");
