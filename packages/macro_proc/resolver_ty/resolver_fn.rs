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
    fn auth(&self) -> Option<String> {
        None
    }
    fn authz(&self) -> Option<String> {
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

        if let Some(auth) = self.auth() {
            if no_ctx {
                self.panic("auth requires ctx");
            }
            let valid = hashset!["authenticated", "unauthenticated"];
            if !valid.contains(&auth.as_ref()) {
                let valid = valid.iter().copied().collect::<Vec<_>>().join(", ");
                let err = format!("invalid auth = {auth}, should be one of: {valid}");
                self.panic(&err);
            }
            let pascal = auth.to_pascal_case().ts2_or_panic();
            let rule = quote!(AuthDirectiveRule::#pascal);
            body = quote! {
                ctx.auth_ensure_in_macro(#rule).await?;
                #body
            };
            directives.push(quote! {
                directive = auth_directive::apply(#rule),
            });
            let shouty = auth.to_shouty_snake_case().ts2_or_panic();
            directive_comments.push(format!("@auth(rule: {shouty})"));
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
