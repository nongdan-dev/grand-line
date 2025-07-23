use crate::prelude::*;

pub trait GenVirtualImpl {
    fn name(&self) -> TokenStream2;
    fn gql_name(&self) -> String;
    fn sql_dep(&self) -> String;
    fn input(&self) -> TokenStream2;
    fn output(&self) -> TokenStream2;
    fn body(&self) -> TokenStream2;

    fn no_async(&self) -> bool {
        false
    }
    fn no_ctx(&self) -> bool {
        false
    }

    fn gen_resolver(&self) -> TokenStream2 {
        let name = self.name();
        let gql_name = self.gql_name();
        let mut input = self.input();
        let output = self.output();
        let mut body = self.body();
        let mut async_modifier = ts2!();
        let mut async_output = output.clone();
        if !self.no_async() {
            async_output = quote!(Result<#output, Box<dyn Error + Send + Sync>>);
            body = quote! {
                let r: #output = {
                    #body
                };
                Ok(r)
            };
            async_modifier = quote!(async);
        }
        if !self.no_ctx() {
            input = quote!(ctx: &async_graphql::Context<'_>, #input);
        }
        quote! {
            // TODO: copy #[graphql...] and comments from the original field
            #[graphql(name=#gql_name)]
            #async_modifier fn #name(&self, #input) -> #async_output {
                #body
            }
        }
    }
}
