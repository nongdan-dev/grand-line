use crate::prelude::*;
use syn::{
    ItemFn, Result, ReturnType,
    parse::{Parse, ParseStream},
};

#[derive(Default)]
pub struct GenResolverTy {
    pub ty: TokenStream2,
    pub name: TokenStream2,
    pub gql_name: String,
    pub inputs: TokenStream2,
    pub output: TokenStream2,
    pub body: TokenStream2,
    pub no_tx: bool,
}

impl GenResolverTy {
    pub fn init(&mut self, a: &MacroAttr, ty_suffix: &str, name_suffix: &str) {
        if self.gql_name == "resolver" {
            if name_suffix == "" {
                panic!("resolver name must be different than the reserved keyword `resolver`");
            }
            self.gql_name = camel_str!(a.model, name_suffix);
        }
        self.name = snake!(self.gql_name);
        self.ty = pascal!(&self.name, ty_suffix);
        self.no_tx = a.no_tx;
    }
}

impl Parse for GenResolverTy {
    fn parse(s: ParseStream) -> Result<Self> {
        let ifn = s.parse::<ItemFn>()?;
        let gql_name = str!(ifn.sig.ident);

        let inputs = ifn.sig.inputs.to_token_stream();
        let output = if let ReturnType::Type(_, ty) = ifn.sig.output {
            ty.to_token_stream()
        } else {
            ts2!("()")
        };

        let body = ifn.block.stmts;
        let body = quote!(#(#body)*);

        let r = GenResolverTy {
            gql_name,
            inputs,
            output,
            body,
            ..Default::default()
        };

        Ok(r)
    }
}

pub fn gen_resolver_ty(g: GenResolverTy) -> TokenStream {
    let GenResolverTy {
        ty,
        name,
        gql_name,
        mut inputs,
        mut output,
        mut body,
        no_tx,
        ..
    } = g;

    inputs = quote!(ctx: &async_graphql::Context<'_>, #inputs);
    output = quote!(Result<#output, Box<dyn Error + Send + Sync>>);

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

    let r = quote! {
        use sea_orm::*;
        use sea_orm::prelude::*;
        use sea_orm::entity::prelude::*;

        #[derive(Default)]
        pub struct #ty;

        #[async_graphql::Object]
        impl #ty {
            // TODO: copy #[graphql...] and comments from the original field
            #[graphql(name=#gql_name)]
            async fn #name(&self, #inputs) -> #output {
                #body
            }
        }
    };

    #[cfg(feature = "debug_macro")]
    debug_macro(&gql_name, r.clone());

    r.into()
}
