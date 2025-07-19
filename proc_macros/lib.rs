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
    DeriveModel,
    attributes(
        // to get attrs from other derive macros
        sea_orm,
        graphql,
        // our attrs
        has_one,
        has_many,
        many_to_many,
        belongs_to,
    )
)]
pub fn derive_model(item: TokenStream) -> TokenStream {
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

#[proc_macro]
pub fn filter(item: TokenStream) -> TokenStream {
    gen_struct(item, "Filter", "Some", "")
}

#[proc_macro]
pub fn filter_some(item: TokenStream) -> TokenStream {
    gen_struct(item, "Filter", "Some", "Some")
}

#[proc_macro]
pub fn active_model(item: TokenStream) -> TokenStream {
    gen_struct(item, "ActiveModel", "sea_orm::ActiveValue::Set", "")
}

#[proc_macro]
pub fn active_create(item: TokenStream) -> TokenStream {
    gen_active_create(item)
}

#[proc_macro]
pub fn active_update(item: TokenStream) -> TokenStream {
    gen_active_update(item)
}
