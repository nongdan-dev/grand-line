use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

pub fn gen_input(_attr: TokenStream, _item: TokenStream) -> TokenStream {
    let item: TokenStream2 = _item.into();

    quote! {
        use grand_line::*;

        #[derive(
          Clone,
          Debug,
          Default,
          serde::Deserialize,
          serde::Serialize,
          async_graphql::InputObject,
        )]
        #[serde_with::skip_serializing_none]
        #item
    }
    .into()
}
