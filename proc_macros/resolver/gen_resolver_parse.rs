use crate::prelude::*;
use syn::{
    ItemFn, Result, ReturnType,
    parse::{Parse, ParseStream},
};

#[derive(Default)]
pub struct GenResolver {
    pub ty: TokenStream2,
    pub name: TokenStream2,
    pub gql_name: String,
    pub inputs: TokenStream2,
    pub output: TokenStream2,
    pub body: TokenStream2,
    pub no_tx: bool,
}

impl GenResolver {
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

impl Parse for GenResolver {
    fn parse(s: ParseStream) -> Result<Self> {
        let ifn = s.parse::<ItemFn>()?;
        let gql_name = str!(ifn.sig.ident);

        let inputs = ifn.sig.inputs.to_token_stream();
        let output = if let ReturnType::Type(_, ref ty) = ifn.sig.output {
            ty.to_token_stream()
        } else {
            ts2!("()")
        };

        let body = ifn.block.stmts;
        let body = quote!(#(#body)*);

        let r = GenResolver {
            gql_name,
            inputs,
            output,
            body,
            ..Default::default()
        };

        Ok(r)
    }
}
