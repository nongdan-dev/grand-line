use crate::prelude::*;

#[field_names]
pub struct CrudAttr {
    pub resolver_inputs: bool,
    pub resolver_output: bool,
    #[field_names(skip)]
    pub model: String,
    #[field_names(skip)]
    pub resolver_attr: ResolverTyAttr,
}
impl From<Attr> for CrudAttr {
    fn from(a: Attr) -> Self {
        attr_unwrap_or_else!(Self {
            resolver_inputs: bool,
            resolver_output: bool,
            model: a.model_from_first_path(),
            resolver_attr: a.into(),
        })
    }
}
impl AttrValidate for CrudAttr {
    fn attr_fields(a: &Attr) -> Vec<String> {
        let mut f = Self::F.iter().map(|f| s!(f)).collect::<Vec<_>>();
        f.extend(ResolverTyAttr::attr_fields(a));
        f.push(a.model_from_first_path());
        f
    }
}

impl CrudAttr {
    pub fn validate(&self, r: &ResolverTyItem) {
        if !self.resolver_inputs && s!(r.inputs) != "" {
            let err = f!(
                "{} inputs must be empty unless resolver_inputs=true, found `{}`",
                r.gql_name,
                r.inputs,
            );
            pan!(err);
        }
        if !self.resolver_output {
            if s!(r.output) != "()" {
                let err = f!(
                    "{} output must be empty unless resolver_output=true, found `{}`",
                    r.gql_name,
                    r.output,
                );
                pan!(err);
            }
            if self.resolver_attr.no_tx || self.resolver_attr.no_ctx {
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
        if !self.resolver_attr.no_tx {
            if self.resolver_attr.no_ctx {
                let err = f!("{} tx requires ctx", r.gql_name);
                pan!(err);
            }
        }
    }
}
