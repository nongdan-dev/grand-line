use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::Ident;

pub fn q_model_filter(
    filter: &Ident,
    filter_fields: &Vec<TokenStream2>,
    filter_matches: &Vec<TokenStream2>,
) -> TokenStream2 {
    quote! {
        use grand_line::*;

        #[input]
        pub struct #filter {
            #(#filter_fields)*
        }
        impl #filter {
            fn query(&self) -> Select<Entity> {
                Entity::find().filter(self.condition())
            }
            fn chain(&self, mut q: Select<Entity>) -> Select<Entity> {
                q.filter(self.condition())
            }
            fn condition(&self) -> Condition {
                let this = self.clone();
                let mut c = Condition::all();
                #(#filter_matches)*
                c
            }
        }
    }
}
