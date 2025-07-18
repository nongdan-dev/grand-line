use crate::prelude::*;
use proc_macro::TokenStream;
use quote::quote;

pub fn gen_active_create(item: TokenStream) -> TokenStream {
    let item = prepend_struct(
        item,
        quote! {
            id: ulid::Ulid::new().to_string(),
            created_at: chrono::Utc::now(),
        },
    );
    gen_struct(item, "ActiveModel", "sea_orm::ActiveValue::Set", "")
}
