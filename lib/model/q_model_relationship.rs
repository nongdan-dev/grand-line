use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

pub fn q_model_relationship() -> TokenStream2 {
    quote! {
        #[derive(Debug, EnumIter, DeriveRelation)]
        pub enum Relation {
            // TODO
        }
    }
}
