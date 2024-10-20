mod crud;
mod derive;
mod input;
mod macro_model;
mod model;
mod resolver;
mod utils;

mod prelude {
    pub use crate::crud::*;
    pub use crate::derive::*;
    pub use crate::input::*;
    pub use crate::macro_model::*;
    pub use crate::model::*;
    pub use crate::resolver::*;
    pub use crate::utils::*;
    pub use grand_line_macros::*;
}

use crate::prelude::*;
use proc_macro::TokenStream;

// ============================================================================
// model

#[proc_macro_attribute]
pub fn model(_attr: TokenStream, _item: TokenStream) -> TokenStream {
    gen_model(_attr, _item)
}

#[proc_macro_derive(
    GrandLineModel,
    attributes(
        // get sea_orm config such as table_name="..." or #[sea_orm(ignore)]
        sea_orm,
        // get graphql config such as name="..." or #[graphql(skip)]
        graphql,
        // our relationship config
        has_one,
        has_many,
        many_to_many,
        belongs_to,
    ),
)]
pub fn grand_line_model(item: TokenStream) -> TokenStream {
    gen_derive(item)
}

#[proc_macro_attribute]
pub fn macro_model(_attr: TokenStream, _item: TokenStream) -> TokenStream {
    gen_macro(_attr, _item)
}

// ============================================================================
// input

#[proc_macro_attribute]
pub fn input(_attr: TokenStream, _item: TokenStream) -> TokenStream {
    gen_input(_attr, _item)
}

#[proc_macro]
pub fn pagination(_item: TokenStream) -> TokenStream {
    gen_pagination(_item)
}

// ============================================================================
// resolver

#[proc_macro_attribute]
pub fn query(_attr: TokenStream, _item: TokenStream) -> TokenStream {
    gen_query(_attr, _item)
}

#[proc_macro_attribute]
pub fn mutation(_attr: TokenStream, _item: TokenStream) -> TokenStream {
    gen_mutation(_attr, _item)
}

// ============================================================================
// crud

#[proc_macro_attribute]
pub fn create(_attr: TokenStream, _item: TokenStream) -> TokenStream {
    gen_create(_attr, _item)
}

#[proc_macro_attribute]
pub fn search(_attr: TokenStream, _item: TokenStream) -> TokenStream {
    gen_search(_attr, _item)
}

#[proc_macro_attribute]
pub fn count(_attr: TokenStream, _item: TokenStream) -> TokenStream {
    gen_count(_attr, _item)
}

#[proc_macro_attribute]
pub fn detail(_attr: TokenStream, _item: TokenStream) -> TokenStream {
    gen_detail(_attr, _item)
}

#[proc_macro_attribute]
pub fn update(_attr: TokenStream, _item: TokenStream) -> TokenStream {
    gen_update(_attr, _item)
}

#[proc_macro_attribute]
pub fn delete(_attr: TokenStream, _item: TokenStream) -> TokenStream {
    gen_delete(_attr, _item)
}

// ============================================================================
// utils

#[proc_macro]
pub fn filter(item: TokenStream) -> TokenStream {
    gen_struct(item, "Filter", "Some", "", false)
}

#[proc_macro]
pub fn filter_some(item: TokenStream) -> TokenStream {
    gen_struct(item, "Filter", "Some", "Some", false)
}

#[proc_macro]
pub fn active_model(item: TokenStream) -> TokenStream {
    gen_struct(item, "ActiveModel", "sea_orm::ActiveValue::Set", "", false)
}

#[proc_macro]
pub fn active_create(item: TokenStream) -> TokenStream {
    gen_active_create(item)
}

#[proc_macro]
pub fn active_update(item: TokenStream) -> TokenStream {
    gen_active_update(item)
}
