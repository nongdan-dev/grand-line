use crate::prelude::*;

pub fn gen_enum(_: TokenStream, item: TokenStream) -> TokenStream {
    let item = Into::<TokenStream2>::into(item);

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
        #item
    }
    .into()
}
