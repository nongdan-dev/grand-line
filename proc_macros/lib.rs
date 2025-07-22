mod crud;
mod input;
mod model;
mod resolver;
mod utils;

mod prelude {
    pub use crate::crud::*;
    pub use crate::input::*;
    pub use crate::model::*;
    pub use crate::resolver::*;
    pub use crate::utils::*;
    pub use grand_line_macros::*;
    pub use heck::*;
    pub use proc_macro::TokenStream;
    pub use proc_macro2::TokenStream as TokenStream2;
    pub use quote::*;
}

use crate::prelude::*;

// ============================================================================
// model

#[proc_macro_attribute]
pub fn model(attr: TokenStream, item: TokenStream) -> TokenStream {
    gen_model(attr, item)
}

#[proc_macro_derive(
    GrandLineModel,
    attributes(
        // get and move them to the graphql output type
        graphql,
        // our attrs
        belongs_to,
        has_one,
        has_many,
        many_to_many,
    )
)]
pub fn grand_line_model(item: TokenStream) -> TokenStream {
    gen_derive(item)
}

// ============================================================================
// input

#[proc_macro_attribute]
pub fn input(attr: TokenStream, item: TokenStream) -> TokenStream {
    gen_input(attr, item)
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

/// Helper to quickly create a filter with concise syntax
#[proc_macro]
pub fn filter(item: TokenStream) -> TokenStream {
    gen_struct(item, "Filter", "Some", "")
}

/// Helper to quickly create a filter with concise syntax and wrap with Some
#[proc_macro]
pub fn filter_some(item: TokenStream) -> TokenStream {
    let item = Into::<TokenStream2>::into(item);
    quote!(Some(filter!(#item))).into()
}

/// Helper to quickly create an order_by with concise syntax
#[proc_macro]
pub fn order_by(item: TokenStream) -> TokenStream {
    gen_order_by(item)
}

/// Helper to quickly create an order_by with concise syntax and wrap with Some
#[proc_macro]
pub fn order_by_some(item: TokenStream) -> TokenStream {
    let item = Into::<TokenStream2>::into(item);
    quote!(Some(order_by!(#item))).into()
}

/// Helper to quickly create an active model with concise syntax
/// and convert all string literals into String automatically
#[proc_macro]
pub fn active_model(item: TokenStream) -> TokenStream {
    gen_struct(item, "ActiveModel", "ActiveValue::Set", "")
}

/// Helper to quickly create an active model with concise syntax
/// and convert all string literals into String automatically.
/// It will also wrap the active model with Entity::active_create
/// to get default values on this operation
#[proc_macro]
pub fn active_create(item: TokenStream) -> TokenStream {
    gen_struct(item, "ActiveModel", "ActiveValue::Set", "active_create")
}

/// Helper to quickly create active model with concise syntax
/// and convert all string literals into String automatically.
/// It will also wrap the active model with Entity::active_update
/// to get default values on this operation
#[proc_macro]
pub fn active_update(item: TokenStream) -> TokenStream {
    gen_struct(item, "ActiveModel", "ActiveValue::Set", "active_update")
}
