mod crud;
mod model;
mod resolver_ty;
mod utils;

#[allow(unused_imports, dead_code)]
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

/// Helper to quickly create a filter with concise syntax and wrap with Some.
#[proc_macro]
pub fn filter_some(item: TokenStream) -> TokenStream {
    let item = Into::<Ts2>::into(item);
    quote!(Some(filter!(#item))).into()
}

/// Helper to quickly create an order_by with concise syntax.
#[proc_macro]
pub fn order_by(item: TokenStream) -> TokenStream {
    gen_order_by(item)
}

/// Helper to quickly create an order_by with concise syntax and wrap with Some.
#[proc_macro]
pub fn order_by_some(item: TokenStream) -> TokenStream {
    let item = Into::<Ts2>::into(item);
    quote!(Some(order_by!(#item))).into()
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

/// Helper to quickly create an active model using am_create
/// then call insert with db and await.
#[proc_macro]
pub fn db_create(item: TokenStream) -> TokenStream {
    gen_db_action("create", "insert", item)
}

/// Helper to quickly create an active model using am_update
/// then call update with db and await.
#[proc_macro]
pub fn db_update(item: TokenStream) -> TokenStream {
    gen_db_action("update", "update", item)
}

/// Helper to quickly create an active model using am_soft_delete
/// then call soft_delete with db and await.
#[proc_macro]
pub fn db_soft_delete(item: TokenStream) -> TokenStream {
    gen_db_action("soft_delete", "soft_delete", item)
}

/// Helper to quickly call soft_delete_by_id with db and await.
#[proc_macro]
pub fn db_soft_delete_by_id(item: TokenStream) -> TokenStream {
    gen_db_soft_delete_by_id(item)
}

/// Helper to quickly call soft_delete_many with db and await.
#[proc_macro]
pub fn db_soft_delete_many(item: TokenStream) -> TokenStream {
    gen_db_soft_delete_many(item)
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
