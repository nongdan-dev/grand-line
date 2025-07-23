use crate::prelude::*;

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
            let gl = GrandLineContext::from(ctx);
            let _tx = gl.tx().await?;
            let tx = _tx.as_ref();
            #body
        };
    }

    quote! {
        use sea_orm::*;
        use sea_orm::prelude::*;
        use sea_orm::entity::prelude::*;

        #[derive(Default)]
        pub struct #ty;

        #[async_graphql::Object]
        impl #ty {
            // TODO: copy #[graphql...] and comments from the original field
            #[graphql(name=#gql_name)]
            async fn #name(
                &self,
                ctx: &async_graphql::Context<'_>,
                #inputs
            ) -> Result<#output, Box<dyn Error + Send + Sync>> {
                #body
            }
        }
    }
    .into()
}
