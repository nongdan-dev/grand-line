use crate::prelude::*;

pub trait GenResolverFn
where
    Self: DebugPanic,
{
    fn name(&self) -> TokenStream2;
    fn gql_name(&self) -> String;
    fn inputs(&self) -> TokenStream2;
    fn output(&self) -> TokenStream2;
    fn body(&self) -> TokenStream2;

    fn no_tx(&self) -> bool {
        false
    }
    fn no_ctx(&self) -> bool {
        false
    }

    fn gen_resolver_fn(&self) -> TokenStream2 {
        let name = self.name();
        let gql_name = self.gql_name();
        let mut inputs = self.inputs();
        let mut output = self.output();
        let mut body = self.body();
        let no_tx = self.no_tx();
        let no_ctx = self.no_ctx();

        if !no_tx {
            if no_ctx {
                self.panic("tx requires ctx");
            }
            body = quote! {
                let _tx = ctx.tx().await?;
                let tx = _tx.as_ref();
                #body
            };
        }

        if !no_ctx {
            inputs = quote!(ctx: &async_graphql::Context<'_>, #inputs);
        }

        body = quote! {
            let r: #output = {
                #body
            };
            Ok(r)
        };
        // TODO: use our error enum to only return client error
        output = quote!(Result<#output, Box<dyn Error + Send + Sync>>);

        quote! {
            // TODO: copy #[graphql...] and comments from the original field
            #[graphql(name=#gql_name)]
            async fn #name(&self, #inputs) -> #output {
                #body
            }
        }
    }
}
