use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::Ident;

pub fn q_model_delete(alias: &Ident, delete: &Ident) -> TokenStream2 {
    quote! {
        pub async fn #delete(
            tx: &DatabaseTransaction,
            id: String,
        ) -> Result<Option<#alias>, Box<dyn std::error::Error + Send + Sync>> {
            match Entity::find_by_id(&id).one(tx).await? {
                Some(v) => {
                    Entity::delete_by_id(&id).exec(tx).await?;
                    Ok(Some(v))
                }
                None => {
                    Ok(None)
                }
            }
        }
    }
}
