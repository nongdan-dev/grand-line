use crate::prelude::*;

#[field_names]
pub struct ResolverTyAttr {
    pub no_tx: bool,
    pub no_ctx: bool,
    pub no_include_deleted: bool,
    #[cfg(feature = "auth")]
    pub auth: String,
    #[field_names(skip)]
    pub inner: Attr,
}
impl From<Attr> for ResolverTyAttr {
    fn from(a: Attr) -> Self {
        attr_unwrap_or_else!(Self {
            no_tx: bool,
            no_ctx: bool,
            #[cfg(feature = "auth")]
            auth: a.str("auth").unwrap_or_default(),
            no_include_deleted: bool,
            inner: a,
        })
    }
}
impl AttrValidate for ResolverTyAttr {
    fn attr_fields(a: &Attr) -> Vec<String> {
        Self::F
            .iter()
            .map(|f| s!(f))
            .filter(|f| {
                if TY_INCLUDE_DELETED.contains(&a.attr) {
                    true
                } else {
                    f != Self::F_NO_INCLUDE_DELETED
                }
            })
            .collect()
    }
}
