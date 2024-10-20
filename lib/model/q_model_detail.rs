use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::Ident;

pub fn q_model_detail(alias: &Ident, detail: &Ident) -> TokenStream2 {
    quote! {
        pub async fn #detail(
            tx: &DatabaseTransaction,
            id: String,
        ) -> Result<#alias, Box<dyn std::error::Error + Send + Sync>> {
            match Entity::find_by_id(&id).one(tx).await? {
                Some(v) => {
                    Ok(v)
                }
                None => {
                    Err("404".into())
                }
            }
        }
    }
}
