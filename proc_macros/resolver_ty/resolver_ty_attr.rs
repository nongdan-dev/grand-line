use crate::prelude::*;
use field_names::FieldNames;

#[derive(FieldNames)]
pub struct ResolverTyAttr {
    pub no_tx: bool,
    pub no_ctx: bool,
    pub no_async: bool,
}

impl ResolverTyAttr {
    pub fn from_without_validate(a: Attr) -> Self {
        let f = Self::FIELDS;
        Self {
            no_tx: a.bool(f[0]),
            no_ctx: a.bool(f[1]),
            no_async: a.bool(f[2]),
        }
    }
    pub fn fields() -> [&'static str; 3] {
        Self::FIELDS
    }
}

impl From<Attr> for ResolverTyAttr {
    fn from(a: Attr) -> Self {
        let f = Self::FIELDS;
        a.validate(f.to_vec());
        Self::from_without_validate(a)
    }
}
