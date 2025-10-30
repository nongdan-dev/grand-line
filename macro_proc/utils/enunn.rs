use crate::prelude::*;

pub fn gen_enunn(_: TokenStream, item: TokenStream) -> TokenStream {
    let item = Into::<Ts2>::into(item);

    quote! {
        #[gql_enum]
        #[derive(EnumIter, DeriveActiveEnum)]
        #[sea_orm(
            rs_type = "String",
            db_type = "String(StringLen::N(255))",
            rename_all = "snake_case"
        )]
        #item
    }
    .into()
}
