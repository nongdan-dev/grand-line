use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

pub fn q_model_active() -> TokenStream2 {
    quote! {
        impl ActiveModelBehavior for ActiveModel {
            fn new() -> Self {
                Self {
                    ..ActiveModelTrait::default()
                }
            }
        }
    }
}
