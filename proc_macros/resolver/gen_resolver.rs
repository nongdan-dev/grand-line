use crate::prelude::*;

#[derive(Default)]
pub struct GenResolver {
    pub ty: TokenStream2,
    pub name: TokenStream2,
    pub gql_name: String,
    pub inputs: TokenStream2,
    pub output: TokenStream2,
    pub body: TokenStream2,
    pub no_tx: bool,
}

pub fn gen_resolver(g: GenResolver) -> TokenStream {
    let GenResolver {
        ty,
        name,
        gql_name,
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

    quote! {
        use grand_line::*;
        use sea_orm::*;
        use sea_orm::entity::prelude::*;

        #[derive(Default)]
        pub struct #ty;

        #[async_graphql::Object]
        impl #ty {
            #[graphql(name=#gql_name)]
            async fn #name(
                &self,
                ctx: &async_graphql::Context<'_>,
                #inputs
            ) -> Result<#output, Box<dyn std::error::Error + Send + Sync>> {
                // TODO: catch panic
                #body
            }
        }
    }
    .into()
}
