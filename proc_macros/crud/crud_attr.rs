use crate::prelude::*;
use field_names::FieldNames;

#[derive(FieldNames)]
pub struct CrudAttr {
    #[field_names(skip)]
    pub model: String,
    #[field_names(skip)]
    pub resolver_attr: ResolverTyAttr,
    pub resolver_inputs: bool,
    pub resolver_output: bool,
}

impl From<Attr> for CrudAttr {
    fn from(a: Attr) -> Self {
        let mut f = Self::FIELDS.to_vec();
        f.extend(ResolverTyAttr::fields());
        a.validate_with_model(f);
        let f = Self::FIELDS;
        Self {
            resolver_inputs: a.bool(f[0]),
            resolver_output: a.bool(f[1]),
            model: a.model_must(),
            resolver_attr: ResolverTyAttr::from_without_validate(a),
        }
    }
}
