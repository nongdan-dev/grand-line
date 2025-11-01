use crate::prelude::*;

pub fn gen_gql_enum(_: TokenStream, item: TokenStream) -> TokenStream {
    let item = Into::<Ts2>::into(item);

    quote! {
        #[derive(
            Debug,
            Clone,
            Eq,
            PartialEq,
            Copy,
            Deserialize,
            Serialize,
            Enum,
        )]
        #item
    }
    .into()
}
