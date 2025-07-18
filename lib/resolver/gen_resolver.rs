use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

#[derive(Default)]
pub struct GenResolver {
    pub ty: TokenStream2,
    pub name: TokenStream2,
    pub name_graphql: String,
    pub inputs_ty: TokenStream2,
    pub inputs: TokenStream2,
    pub output: TokenStream2,
    pub body: TokenStream2,
    pub no_tx: bool,
}

pub fn gen_resolver(g: GenResolver) -> TokenStream {
    let GenResolver {
        ty,
        name,
        name_graphql,
        inputs_ty,
        inputs,
        output,
        mut body,
        no_tx,
        ..
    } = g;

    body = quote! {
        Ok({ #body })
    };

    if !no_tx {
        body = quote! {
            let tx = ctx.data_unchecked::<DatabaseConnection>().begin().await?;
            let r = #body;
            tx.commit().await?;
            r
        };
    }

    let resolver = quote! {
        use grand_line::*;
        use sea_orm::*;
        use sea_orm::entity::prelude::*;

        #[derive(Default)]
        pub struct #ty;

        #[async_graphql::Object]
        impl #ty {
            #[graphql(name=#name_graphql)]
            async fn #name(
                &self,
                ctx: &async_graphql::Context<'_>,
                #inputs
            ) -> Result<#output, Box<dyn std::error::Error + Send + Sync>> {
                // TODO: catch panic
                #body
            }
        }
    };

    quote! {
        #inputs_ty
        #resolver
    }
    .into()
}
