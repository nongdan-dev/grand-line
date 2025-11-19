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
    fn auth(&self) -> Option<AuthAttr> {
        None
    }
    fn authz(&self) -> Option<AuthzAttr> {
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

        let (mut directives, mut directive_comments) = (vec![], vec![]);

        if let Some(AuthAttr { unauthenticated }) = self.auth() {
            if no_ctx {
                self.panic("auth requires ctx");
            }
            let check_str = if unauthenticated {
                "unauthenticated"
            } else {
                "authenticated"
            };
            let pascal = check_str.to_pascal_case().ts2_or_panic();
            let check = quote!(AuthDirectiveCheck::#pascal);
            body = quote! {
                ctx.auth_ensure_in_macro(#check).await?;
                #body
            };
            directives.push(quote! {
                directive = auth_directive::apply(#check),
            });
            let shouty = check_str.to_shouty_snake_case().ts2_or_panic();
            directive_comments.push(format!("@auth(check: {shouty})"));
        }
        if let Some(AuthzAttr { org, user, key }) = self.authz() {
            if no_ctx {
                self.panic("authz requires ctx");
            }
            let key = if let Some(key) = key {
                quote!(Some(#key.to_owned()))
            } else {
                quote!(None)
            };
            body = quote! {
                ctx.authz_ensure_in_macro(AuthzDirectiveEnsure {
                    org: #org,
                    user: #user,
                    key: #key,
                }).await?;
                #body
            };
            let mut checks = vec![];
            if org {
                checks.push(quote!(AuthzDirectiveCheck::Org,));
            }
            if user {
                checks.push(quote!(AuthzDirectiveCheck::User,));
            }
            let check = quote!(vec![#(#checks)*]);
            directives.push(quote! {
                directive = authz_directive::apply(#check, #key),
            });
            // TODO: directive_comments
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
                name = #gql_name,
                #(#directives)*
            )]
            #(#[doc = #directive_comments])*
            async fn #name(&self, #inputs) -> #output {
                #body
            }
        }
    }
}
