use crate::prelude::*;

pub fn gen_gql_enum(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr = Into::<Ts2>::into(attr);
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
        #attr
        #item
    }
    .into()
}
