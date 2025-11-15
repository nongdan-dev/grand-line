use crate::prelude::*;

pub trait ResolverFn
where
    Self: AttrDebug,
{
    fn name(&self) -> Ts2;
    fn gql_name(&self) -> String;
    fn inputs(&self) -> Ts2;
    fn output(&self) -> Ts2;
    fn body(&self) -> Ts2;

    fn no_tx(&self) -> bool {
        false
    }
    fn no_ctx(&self) -> bool {
        false
    }
    #[cfg(feature = "auth")]
    fn auth(&self) -> String {
        "".to_owned()
    }

    fn resolver_fn(&self) -> Ts2 {
        let name = self.name();
        let gql_name = self.gql_name();
        let mut inputs = self.inputs();
        let mut output = self.output();
        let mut body = self.body();
        let no_tx = self.no_tx();
        let no_ctx = self.no_ctx();

        #[cfg(feature = "auth")]
        {
            if no_ctx {
                let err = self.err("auth requires ctx");
                pan!(err);
            }
            let auth = self.auth();
            let valid_auth = ["none", "authenticate", "unauthenticated"];
            if !auth.is_empty() && !valid_auth.contains(&auth.as_str()) {
                let err = f!("auth should be one of: {}", valid_auth.join(", "));
                let err = self.err(&err);
                pan!(err)
            }
            let auth = if auth.is_empty() {
                quote!(None)
            } else {
                let auth = pascal!(auth);
                quote!(Some(AuthEnsure::#auth))
            };
            body = quote! {
                ctx.ensure_auth_in_macro(#auth).await?;
                #body
            };
        }

        if !no_tx {
            if no_ctx {
                let err = self.err("tx requires ctx");
                pan!(err);
            }
            body = quote! {
                let tx = &*ctx.tx().await?;
                #body
            };
        }

        if !no_ctx {
            inputs = quote!(ctx: &Context<'_>, #inputs);
        }

        body = quote! {
            let r: #output = {
                #body
            };
            Ok(r)
        };
        output = quote!(Res<#output>);

        quote! {
            // TODO: copy #[graphql...] and comments from the original field
            #[graphql(name=#gql_name)]
            async fn #name(&self, #inputs) -> #output {
                #body
            }
        }
    }
}
