use crate::prelude::*;

pub fn gen_input(_: TokenStream, item: TokenStream) -> TokenStream {
    let item = Into::<TokenStream2>::into(item);

    quote! {
        #[serde_with::skip_serializing_none]
        #[derive(
          Clone,
          Debug,
          Default,
          serde::Deserialize,
          serde::Serialize,
          async_graphql::InputObject,
        )]
        #item
    }
    .into()
}
