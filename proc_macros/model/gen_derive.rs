use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;

pub fn gen_derive(_: TokenStream) -> TokenStream {
    "".parse::<TokenStream2>().unwrap().into()
}
