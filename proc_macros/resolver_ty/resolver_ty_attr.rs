use crate::prelude::*;
use field_names::FieldNames;

#[derive(Debug, Clone, Default, FieldNames)]
pub struct ResolverTyAttr {
    pub no_tx: bool,
    pub no_ctx: bool,
    pub no_async: bool,
}
impl From<Attr> for ResolverTyAttr {
    fn from(a: Attr) -> Self {
        attr_unwrap!(Self {
            no_tx: bool,
            no_ctx: bool,
            no_async: bool,
        })
    }
}
impl AttrValidate for ResolverTyAttr {
    fn attr_fields(_: &Attr) -> Vec<String> {
        Self::FIELDS.iter().map(|f| str!(f)).collect()
    }
}
