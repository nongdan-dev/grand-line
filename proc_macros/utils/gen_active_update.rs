use crate::prelude::*;
use proc_macro::TokenStream;
use quote::quote;

pub fn gen_active_update(item: TokenStream) -> TokenStream {
    let item = prepend_struct(
        item,
        quote! {
            updated_at: Some(chrono::Utc::now()),
        },
    );
    gen_struct(item, "ActiveModel", "sea_orm::ActiveValue::Set", "")
}
