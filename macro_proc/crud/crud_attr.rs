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
        if !self.resolver_inputs && !s!(r.inputs).is_empty() {
            let err = f!(
                "{} inputs should be empty unless resolver_inputs=true, found {}",
                r.gql_name,
                r.inputs,
            );
            pan!(err);
        }
        if !self.resolver_output {
            if s!(r.output) != "()" {
                let err = f!(
                    "{} output should be empty unless resolver_output=true, found {}",
                    r.gql_name,
                    r.output,
                );
                pan!(err);
            }
            if self.ra.no_tx || self.ra.no_ctx {
                let err = f!("{} output requires tx, ctx", r.gql_name);
                pan!(err);
            }
        }
        if self.resolver_inputs && self.resolver_output {
            let err = f!(
                "{} should use #[query] or #[mutation] instead since both resolver_inputs=true and resolver_output=true",
                r.gql_name,
            );
            pan!(err);
        }
        if !self.ra.no_tx && self.ra.no_ctx {
            let err = f!("{} tx requires ctx", r.gql_name);
            pan!(err);
        }
    }
}
