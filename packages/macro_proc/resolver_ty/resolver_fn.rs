use crate::prelude::*;

pub trait ResolverFn
where
    Self: AttrDebug,
{
    fn name(&self) -> SynRes<Ts2>;
    fn gql_name(&self) -> SynRes<String>;
    fn inputs(&self) -> SynRes<Ts2>;
    fn output(&self) -> SynRes<Ts2>;
    fn body(&self) -> SynRes<Ts2>;

    fn root_operation_ty(&self) -> SynRes<Option<String>> {
        Ok(None)
    }

    fn tx(&self) -> bool {
        true
    }
    fn ctx(&self) -> bool {
        true
    }
    fn auth(&self) -> Option<AuthAttr> {
        None
    }
    fn authz(&self) -> Option<AuthzAttr> {
        None
    }

    /// Doc-comment strings from the original field definition.
    /// Each entry corresponds to one `///` line (with leading space preserved).
    fn doc_strs(&self) -> Vec<String> {
        vec![]
    }

    /// Extra `#[graphql(...)]` args (everything except `name`) from the
    /// original field definition. Already formatted with trailing commas,
    /// ready to be spliced into the generated graphql attribute.
    fn extra_graphql(&self) -> Ts2 {
        quote!()
    }

    fn resolver_fn(&self) -> SynRes<Ts2> {
        let name = self.name()?;
        let gql_name = self.gql_name()?;
        let mut inputs = self.inputs()?;
        let mut output = self.output()?;
        let mut body = self.body()?;
        let tx = self.tx();
        let ctx = self.ctx();

        let (mut directives, mut directive_comments) = (vec![], vec![]);

        if let Some(a) = self.auth() {
            if !ctx {
                return Err(self.syn_err("auth requires ctx"));
            }
            let check_str = if a.unauthenticated {
                "unauthenticated"
            } else {
                "authenticated"
            };
            let pascal = check_str.to_pascal_case().ts2_or_err()?;
            let check = quote!(AuthDirectiveCheck::#pascal);
            body = quote! {
                ctx.auth_ensure_in_macro(#check).await?;
                #body
            };
            directives.push(quote! {
                directive = auth_directive::apply(#check),
            });
            let shouty = check_str.to_shouty_snake_case().ts2_or_err()?;
            directive_comments.push(format!("@auth(check: {shouty})"));
        }

        if let Some(AuthzAttr {
            realm,
            skip_org,
            skip_user,
        }) = self.authz()
        {
            if !ctx {
                return Err(self.syn_err("authz requires ctx"));
            }
            let org = !skip_org;
            let user = !skip_user;
            let operation_ty = self
                .root_operation_ty()?
                .ok_or_else(|| self.syn_err("authz only available in root resolvers"))?
                .ts2_or_err()?;
            let realm = quote!(#realm.to_owned());
            body = quote! {
                ctx.cache(async || {
                    Ok(AuthzCacheOperationTy::#operation_ty)
                })
                .await?;
                ctx.authz_ensure_in_macro(AuthzDirectiveEnsure {
                    realm: #realm,
                    org: #org,
                    user: #user,
                })
                .await?;
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
                directive = authz_directive::apply(#realm, #check),
            });
            let mut checks = vec![];
            if org {
                checks.push("ORG");
            }
            if user {
                checks.push("USER");
            }
            let checks = checks.join(", ");
            let realm = realm.to_token_stream().to_string();
            directive_comments.push(format!("@authz(realm: {realm}, check: [{checks}])"));
        }

        if tx {
            if !ctx {
                return Err(self.syn_err("tx requires ctx"));
            }
            body = quote! {
                let tx = &*ctx.tx().await?;
                #body
            };
        }

        if ctx {
            inputs = quote!(ctx: &Context<'_>, #inputs);
        }

        body = quote! {
            let r: #output = {
                #body
            };
            Ok(r)
        };
        output = quote!(Res<#output>);

        let field_doc_strs = self.doc_strs();
        let field_extra = self.extra_graphql();
        let graphql_attr = if field_extra.is_empty() {
            quote!(#[graphql(name = #gql_name, #(#directives)*)])
        } else {
            quote!(#[graphql(name = #gql_name, #(#directives)* #field_extra)])
        };

        Ok(quote! {
            #(#[doc = #field_doc_strs])*
            #graphql_attr
            #(#[doc = #directive_comments])*
            async fn #name(&self, #inputs) -> #output {
                #body
            }
        })
    }
}
