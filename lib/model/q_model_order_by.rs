use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::Ident;

pub fn q_model_order_by(
    order_by: &Ident,
    order_by_fields: &Vec<TokenStream2>,
    order_by_matches: &Vec<TokenStream2>,
) -> TokenStream2 {
    quote! {
        #[derive(
          Clone,
          Debug,
          Copy,
          Eq,
          PartialEq,
          serde::Deserialize,
          serde::Serialize,
          async_graphql::Enum,
        )]
        pub enum #order_by {
            #(#order_by_fields)*
        }
        impl #order_by {
            fn chain(&self, q: Select<Entity>) -> Select<Entity> {
                match *self {
                    #(#order_by_matches)*
                }
            }
        }
    }
}
