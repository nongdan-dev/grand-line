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
        attr_unwrap_or_else!(Self {
            resolver_inputs: bool,
            resolver_output: bool,
            no_permanent_delete: bool,
            model: a.model_from_first_path(),
            ra: a.into(),
        })
    }
}
impl AttrValidate for CrudAttr {
    fn attr_fields(a: &Attr) -> Vec<String> {
        Self::F
            .iter()
            .map(|f| s!(f))
            .filter(|f| {
                if a.attr == MacroTy::Delete {
                    true
                } else {
                    f != Self::F_NO_PERMANENT_DELETE
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
        if !self.resolver_inputs && !s!(inputs).is_empty() {
            pan!("{gql_name} inputs should be empty unless resolver_inputs=true, found {inputs}");
        }
        if !self.resolver_output {
            if s!(output) != "()" {
                pan!(
                    "{gql_name} output should be empty unless resolver_output=true, found {output}",
                );
            }
            if self.ra.no_tx || self.ra.no_ctx {
                pan!("{gql_name} output requires tx, ctx");
            }
        }
        if self.resolver_inputs && self.resolver_output {
            pan!(
                "{gql_name} should use #[query] or #[mutation] instead since both resolver_inputs=true and resolver_output=true",
            );
        }
        if !self.ra.no_tx && self.ra.no_ctx {
            pan!("{gql_name} tx requires ctx");
        }
    }
}
