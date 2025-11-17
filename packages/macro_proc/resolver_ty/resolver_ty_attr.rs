use crate::prelude::*;

#[field_names]
pub struct ResolverTyAttr {
    pub no_tx: bool,
    pub no_ctx: bool,
    pub no_include_deleted: bool,
    #[cfg(feature = "auth")]
    pub auth: Option<bool>,
    #[field_names(skip)]
    pub inner: Attr,
}
impl From<Attr> for ResolverTyAttr {
    fn from(a: Attr) -> Self {
        Self {
            no_tx: a.bool(Self::F_NO_TX).unwrap_or(FEATURE_NO_TX),
            no_ctx: a.bool(Self::F_NO_CTX).unwrap_or(FEATURE_NO_CTX),
            #[cfg(feature = "auth")]
            auth: a.bool(Self::F_AUTH),
            no_include_deleted: a
                .bool(Self::F_NO_INCLUDE_DELETED)
                .unwrap_or(FEATURE_NO_INCLUDE_DELETED),
            inner: a,
        }
    }
}
impl AttrValidate for ResolverTyAttr {
    fn attr_fields(a: &Attr) -> Vec<String> {
        Self::F
            .iter()
            .map(|f| (*f).to_owned())
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
