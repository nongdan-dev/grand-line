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
