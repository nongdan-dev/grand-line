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
        s!()
    }

    fn resolver_fn(&self) -> Ts2 {
        let name = self.name();
        let gql_name = self.gql_name();
        let mut inputs = self.inputs();
        let mut output = self.output();
        let mut body = self.body();
        let no_tx = self.no_tx();
        let no_ctx = self.no_ctx();

        #[cfg(not(any(feature = "auth")))]
        let (directives, comments): (Vec<Ts2>, Vec<Ts2>) = (vec![], vec![]);
        #[cfg(any(feature = "auth", feature = "guard"))]
        let (mut directives, mut comments) = (vec![], vec![]);

        #[cfg(feature = "auth")]
        {
            if no_ctx {
                let err = self.err("auth requires ctx");
                pan!("{err}");
            }
            let mut auth = self.auth();
            if auth.is_empty() {
                auth = "none".to_owned();
            }
            let valid = ["none", "authenticate", "unauthenticated"];
            if !valid.contains(&auth.as_str()) {
                let valid = valid.join(", ");
                let err = f!("auth should be one of: {valid}");
                let err = self.err(&err);
                pan!("{err}");
            }
            let auth_pascal = pascal!(auth);
            let ensure = quote!(AuthEnsure::#auth_pascal);
            body = quote! {
                ctx.ensure_auth_in_macro(#ensure).await?;
                #body
            };
            directives.push(quote! {
                directive=auth_directive::apply(#ensure),
            });
            let auth_scream = scream!(auth);
            comments.push(f!("@auth(ensure: {auth_scream})"));
        }

        if !no_tx {
            if no_ctx {
                let err = self.err("tx requires ctx");
                pan!("{err}");
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
            #[graphql(
                name=#gql_name,
                #(#directives)*
            )]
            #(#[doc = #comments])*
            async fn #name(&self, #inputs) -> #output {
                #body
            }
        }
    }
}
