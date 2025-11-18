use crate::prelude::*;
use std::iter::once;

#[field_names]
pub struct CrudAttr {
    pub resolver_inputs: bool,
    pub resolver_output: bool,
    pub no_permanent_delete: bool,
    #[field_names(skip)]
    pub model: String,
    #[field_names(skip)]
    pub ra: ResolverTyAttr,
}
impl From<Attr> for CrudAttr {
    fn from(a: Attr) -> Self {
        Self {
            resolver_inputs: a
                .bool(Self::FIELD_RESOLVER_INPUTS)
                .unwrap_or(FEATURE_RESOLVER_INPUTS),
            resolver_output: a
                .bool(Self::FIELD_RESOLVER_OUTPUT)
                .unwrap_or(FEATURE_RESOLVER_OUTPUT),
            no_permanent_delete: a
                .bool(Self::FIELD_NO_PERMANENT_DELETE)
                .unwrap_or(FEATURE_NO_PERMANENT_DELETE),
            model: a.model_from_first_path(),
            ra: a.into(),
        }
    }
}
impl AttrValidate for CrudAttr {
    fn attr_fields(a: &Attr) -> Vec<String> {
        Self::F
            .iter()
            .copied()
            .map(|f| f.to_owned())
            .filter(|f| {
                if a.attr == MacroTy::Delete {
                    true
                } else {
                    f != Self::FIELD_NO_PERMANENT_DELETE
                }
            })
            .chain(ResolverTyAttr::attr_fields(a))
            .chain(once(a.model_from_first_path()))
            .collect()
    }
}

impl CrudAttr {
    pub fn validate(&self, r: &ResolverTyItem) {
        let ResolverTyItem {
            gql_name,
            inputs,
            output,
            ..
        } = &r;
        if !self.resolver_inputs && !inputs.to_string().is_empty() {
            panic!("{gql_name} inputs should be empty unless resolver_inputs=true, found {inputs}");
        }
        if !self.resolver_output {
            if output.to_string() != "()" {
                panic!(
                    "{gql_name} output should be empty unless resolver_output=true, found {output}",
                );
            }
            if self.ra.no_tx || self.ra.no_ctx {
                panic!("{gql_name} output requires tx, ctx");
            }
        }
        if self.resolver_inputs && self.resolver_output {
            panic!(
                "{gql_name} should use #[query] or #[mutation] instead since both resolver_inputs=true and resolver_output=true",
            );
        }
        if !self.ra.no_tx && self.ra.no_ctx {
            panic!("{gql_name} tx requires ctx");
        }
    }
}
