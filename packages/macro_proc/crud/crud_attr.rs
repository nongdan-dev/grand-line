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
impl TryFrom<Attr> for CrudAttr {
    type Error = SynErr;
    fn try_from(a: Attr) -> SynRes<Self> {
        Ok(Self {
            resolver_inputs: a
                .bool(Self::FIELD_RESOLVER_INPUTS)?
                .unwrap_or(FEATURE_RESOLVER_INPUTS),
            resolver_output: a
                .bool(Self::FIELD_RESOLVER_OUTPUT)?
                .unwrap_or(FEATURE_RESOLVER_OUTPUT),
            no_permanent_delete: a
                .bool(Self::FIELD_NO_PERMANENT_DELETE)?
                .unwrap_or(FEATURE_NO_PERMANENT_DELETE),
            model: a.model_from_first_path()?,
            ra: a.clone().try_into()?,
        })
    }
}
impl AttrValidate for CrudAttr {
    fn attr_fields(a: &Attr) -> Vec<String> {
        Self::FIELDS
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
            .chain(once(a.model_from_first_path().unwrap_or_default()))
            .collect()
    }
}

impl CrudAttr {
    pub fn validate(&self, r: &ResolverTyItem) -> SynRes<()> {
        let ResolverTyItem {
            gql_name,
            inputs,
            output,
            span,
            ..
        } = &r;
        if !self.resolver_inputs && !inputs.to_string().is_empty() {
            let err = format!(
                "{gql_name} inputs should be empty unless resolver_inputs=true, found {inputs}",
            );
            return Err(SynErr::new(*span, err));
        }
        if !self.resolver_output {
            if output.to_string() != "()" {
                let err = format!(
                    "{gql_name} output should be empty unless resolver_output=true, found {output}",
                );
                return Err(SynErr::new(*span, err));
            }
            if self.ra.no_tx || self.ra.no_ctx {
                let err = format!("{gql_name} output requires tx, ctx");
                return Err(SynErr::new(*span, err));
            }
        }
        if self.resolver_inputs && self.resolver_output {
            let err = format!(
                "{gql_name} should use #[query] or #[mutation] instead since both resolver_inputs=true and resolver_output=true",
            );
            return Err(SynErr::new(*span, err));
        }
        if !self.ra.no_tx && self.ra.no_ctx {
            let err = format!("{gql_name} tx requires ctx");
            return Err(SynErr::new(*span, err));
        }
        Ok(())
    }
}
