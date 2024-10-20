use proc_macro::TokenStream;
use quote::quote;

pub fn gen_pagination(_item: TokenStream) -> TokenStream {
    quote! {
        use grand_line::*;

        #[input]
        pub struct Pagination {
            pub limit: Option<u64>,
            pub offset: Option<u64>,
        }
    }
    .into()
}
