mod crud;
mod model;
mod resolver_ty;
mod utils;

#[allow(ambiguous_glob_reexports, dead_code, unused_imports)]
mod prelude {
    pub use crate::crud::*;
    pub use crate::model::*;
    pub use crate::resolver_ty::*;
    pub use crate::utils::*;
    pub use _utils::*;
    pub use _utils_proc::*;
    pub use proc_macro::TokenStream;
    use_common_macro_utils!();
    use_common_std!();
}
use crate::prelude::*;

// ============================================================================
// model

#[proc_macro_attribute]
pub fn model(attr: TokenStream, item: TokenStream) -> TokenStream {
    gen_model(attr, item)
}

// ============================================================================
// resolver

#[proc_macro_attribute]
pub fn query(attr: TokenStream, item: TokenStream) -> TokenStream {
    gen_query(attr, item)
}

#[proc_macro_attribute]
pub fn mutation(attr: TokenStream, item: TokenStream) -> TokenStream {
    gen_mutation(attr, item)
}

// ============================================================================
// crud

#[proc_macro_attribute]
pub fn create(attr: TokenStream, item: TokenStream) -> TokenStream {
    gen_create(attr, item)
}

#[proc_macro_attribute]
pub fn search(attr: TokenStream, item: TokenStream) -> TokenStream {
    gen_search(attr, item)
}

#[proc_macro_attribute]
pub fn count(attr: TokenStream, item: TokenStream) -> TokenStream {
    gen_count(attr, item)
}

#[proc_macro_attribute]
pub fn detail(attr: TokenStream, item: TokenStream) -> TokenStream {
    gen_detail(attr, item)
}

#[proc_macro_attribute]
pub fn update(attr: TokenStream, item: TokenStream) -> TokenStream {
    gen_update(attr, item)
}

#[proc_macro_attribute]
pub fn delete(attr: TokenStream, item: TokenStream) -> TokenStream {
    gen_delete(attr, item)
}

// ============================================================================
// utils

#[proc_macro_attribute]
pub fn enunn(attr: TokenStream, item: TokenStream) -> TokenStream {
    gen_enunn(attr, item)
}

#[proc_macro_attribute]
pub fn gql_enum(attr: TokenStream, item: TokenStream) -> TokenStream {
    gen_gql_enum(attr, item)
}

#[proc_macro_attribute]
pub fn gql_input(attr: TokenStream, item: TokenStream) -> TokenStream {
    gen_gql_input(attr, item)
}

/// Helper to quickly create a filter with concise syntax.
#[proc_macro]
pub fn filter(item: TokenStream) -> TokenStream {
    expr_struct(item, "Filter", "Some", "")
}

/// Helper to quickly create an order_by with concise syntax.
#[proc_macro]
pub fn order_by(item: TokenStream) -> TokenStream {
    gen_order_by(item)
}

/// Helper to quickly create an active model with concise syntax
/// and convert all string literals into String automatically.
#[proc_macro]
pub fn active_model(item: TokenStream) -> TokenStream {
    expr_struct(item, "ActiveModel", "Set", "")
}

/// Helper to quickly create an active model with concise syntax
/// and convert all string literals into String automatically.
/// It will also wrap the active model with ActiveModelX::set_defaults_on_create
/// to get default values on this operation.
#[proc_macro]
pub fn am_create(item: TokenStream) -> TokenStream {
    expr_struct(item, "ActiveModel", "Set", "set_defaults_on_create")
}

/// Helper to quickly create active model with concise syntax
/// and convert all string literals into String automatically.
/// It will also wrap the active model with ActiveModelX::set_defaults_on_update
/// to get default values on this operation.
#[proc_macro]
pub fn am_update(item: TokenStream) -> TokenStream {
    expr_struct(item, "ActiveModel", "Set", "set_defaults_on_update")
}

/// Helper to quickly create active model with concise syntax
/// and convert all string literals into String automatically.
/// It will also wrap the active model with ActiveModelX::set_defaults_on_delete
/// to get default values on this operation.
#[proc_macro]
pub fn am_soft_delete(item: TokenStream) -> TokenStream {
    expr_struct(item, "ActiveModel", "Set", "set_defaults_on_delete")
}

/// Automatically derive ThisErr, GrandLineErrDerive, Debug.
#[proc_macro_attribute]
pub fn grand_line_err(attr: TokenStream, item: TokenStream) -> TokenStream {
    gen_grand_line_err(attr, item)
}

/// Automatically implement GrandLineErrImpl to handle error better.
#[proc_macro_derive(GrandLineErrDerive, attributes(client, code))]
pub fn grand_line_err_derive(item: TokenStream) -> TokenStream {
    gen_grand_line_err_derive(item)
}
