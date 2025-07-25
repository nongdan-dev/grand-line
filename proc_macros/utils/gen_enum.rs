use crate::prelude::*;

pub fn gen_enum(_: TokenStream, item: TokenStream) -> TokenStream {
    let item = Into::<TokenStream2>::into(item);

    quote! {
        #[derive(
            Debug,
            Clone,
            Eq,
            PartialEq,
            Copy,
            serde::Deserialize,
            serde::Serialize,
            async_graphql::Enum,
        )]
        #item
    }
    .into()
}
