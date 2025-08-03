use crate::prelude::*;

#[field_names]
pub struct ResolverTyAttr {
    pub no_tx: bool,
    pub no_ctx: bool,
    #[field_names(skip)]
    pub inner: Attr,
}
impl From<Attr> for ResolverTyAttr {
    fn from(a: Attr) -> Self {
        attr_unwrap_or_else!(Self {
            no_tx: bool,
            no_ctx: bool,
            inner: a,
        })
    }
}
impl AttrValidate for ResolverTyAttr {
    fn attr_fields(_: &Attr) -> Vec<String> {
        Self::F.iter().map(|f| s!(f)).collect()
    }
}
