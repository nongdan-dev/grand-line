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
                panic!("resolver name should be different than the reserved keyword `resolver`");
            }
            if crud_model.is_empty() {
                panic!("empty model name should be already validated at the previous step");
            }
            self.gql_name = format!("{crud_model}_{crud}").to_lower_camel_case();
        }
        let name = self.gql_name.to_snake_case().ts2_or_panic();
        let ty = format!("{name}_{operation}")
            .to_pascal_case()
            .ts2_or_panic();
        (self, ty, name)
    }
}

impl Parse for ResolverTyItem {
    fn parse(s: ParseStream) -> Result<Self> {
        let ifn = s.parse::<ItemFn>()?;
        let gql_name = ifn.sig.ident.to_string().to_lower_camel_case();

        let inputs = ifn.sig.inputs.to_token_stream();
        let output = if let ReturnType::Type(_, ty) = ifn.sig.output {
            ty.to_token_stream()
        } else {
            quote!(())
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
