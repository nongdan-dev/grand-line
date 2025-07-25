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
        let f = Self::FIELDS;
        Self {
            no_tx: a.bool(f[0]),
            no_ctx: a.bool(f[1]),
            no_async: a.bool(f[2]),
        }
    }
}
impl AttrValidate for ResolverTyAttr {
    fn attr_fields(_: &Attr) -> Vec<String> {
        Self::FIELDS.iter().map(|f| str!(f)).collect()
    }
}
