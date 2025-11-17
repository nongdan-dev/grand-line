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
    fn auth(&self) -> Option<bool> {
        None
    }

    fn resolver_fn(&self) -> Ts2 {
        let name = self.name();
        let gql_name = self.gql_name();
        let mut inputs = self.inputs();
        let mut output = self.output();
        let mut body = self.body();
        let no_tx = self.no_tx();
        let no_ctx = self.no_ctx();

        #[cfg(not(any(feature = "auth", feature = "policy")))]
        let (directives, directive_comments): (Vec<Ts2>, Vec<Ts2>) = (vec![], vec![]);
        #[cfg(any(feature = "auth", feature = "policy"))]
        let (mut directives, mut directive_comments) = (vec![], vec![]);

        #[cfg(feature = "auth")]
        {
            if no_ctx {
                self.panic("auth requires ctx");
            }
            let auth = match self.auth() {
                Some(true) => "authenticate",
                Some(false) => "unauthenticated",
                None => "none",
            };
            let pascal = auth.to_pascal_case().ts2_or_panic();
            let ensure = quote!(AuthEnsure::#pascal);
            body = quote! {
                ctx.ensure_auth_in_macro(#ensure).await?;
                #body
            };
            directives.push(quote! {
                directive=auth_directive::apply(#ensure),
            });
            let shouty = auth.to_shouty_snake_case().ts2_or_panic();
            directive_comments.push(format!("@auth(ensure: {shouty})"));
        }

        if !no_tx {
            if no_ctx {
                self.panic("tx requires ctx");
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
            // TODO: copy #[graphql...] and directive_comments from the original field
            #[graphql(
                name=#gql_name,
                #(#directives)*
            )]
            #(#[doc = #directive_comments])*
            async fn #name(&self, #inputs) -> #output {
                #body
            }
        }
    }
}
