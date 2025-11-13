use crate::prelude::*;

#[derive(Default)]
pub struct ResolverTyItem {
    pub gql_name: String,
    pub inputs: Ts2,
    pub output: Ts2,
    pub body: Ts2,
}

impl ResolverTyItem {
    pub fn init(mut self, operation: &str, crud: &str, crud_model: &str) -> (Self, Ts2, Ts2) {
        if self.gql_name == "resolver" {
            if crud.is_empty() {
                let err = "resolver name should be different than the reserved keyword `resolver`";
                pan!(err);
            }
            if crud_model.is_empty() {
                let err = "empty model name should be validated earlier";
                pan!(err);
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
        let gql_name = s!(ifn.sig.ident);

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
