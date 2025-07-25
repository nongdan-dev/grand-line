use crate::prelude::*;
use field_names::FieldNames;

#[derive(Debug, Clone, Default, FieldNames)]
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
        let f = Self::FIELDS;
        Self {
            resolver_inputs: a.bool(f[0]),
            resolver_output: a.bool(f[1]),
            model: a.model_must(),
            resolver_attr: a.into(),
        }
    }
}
impl AttrValidate for CrudAttr {
    fn attr_fields(a: &Attr) -> Vec<String> {
        let mut f = Self::FIELDS.iter().map(|f| str!(f)).collect::<Vec<_>>();
        f.extend(ResolverTyAttr::attr_fields(a));
        f.push(a.model_must());
        f
    }
}
