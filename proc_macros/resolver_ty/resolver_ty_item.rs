use crate::prelude::*;
use syn::{
    ItemFn, Result, ReturnType,
    parse::{Parse, ParseStream},
};

#[derive(Default)]
pub struct ResolverTyItem {
    pub gql_name: String,
    pub inputs: TokenStream2,
    pub output: TokenStream2,
    pub body: TokenStream2,
}

impl ResolverTyItem {
    pub fn init(
        mut self,
        operation: &str,
        crud: &str,
        crud_model: &str,
    ) -> (Self, TokenStream2, TokenStream2) {
        if self.gql_name == "resolver" {
            if crud == "" {
                panic!("resolver name must be different than the reserved keyword `resolver`");
            }
            if crud_model == "" {
                panic!("empty model name should be validated earlier");
            }
            self.gql_name = camel_str!(crud_model, crud);
        }
        let name = snake!(self.gql_name);
        let ty = pascal!(name, operation);
        (self, ty, name)
    }
}

impl Parse for ResolverTyItem {
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

        let r = Self {
            gql_name,
            inputs,
            output,
            body,
        };

        Ok(r)
    }
}
