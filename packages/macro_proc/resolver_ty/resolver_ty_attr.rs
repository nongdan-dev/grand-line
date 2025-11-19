use crate::prelude::*;

#[field_names]
pub struct ResolverTyAttr {
    pub no_tx: bool,
    pub no_ctx: bool,
    pub no_include_deleted: bool,
    pub auth: Option<AuthAttr>,
    pub authz: Option<AuthzAttr>,
    #[field_names(skip)]
    pub inner: Attr,
}
impl From<Attr> for ResolverTyAttr {
    fn from(a: Attr) -> Self {
        Self {
            no_tx: a.bool(Self::FIELD_NO_TX).unwrap_or(FEATURE_NO_TX),
            no_ctx: a.bool(Self::FIELD_NO_CTX).unwrap_or(FEATURE_NO_CTX),
            no_include_deleted: a
                .bool(Self::FIELD_NO_INCLUDE_DELETED)
                .unwrap_or(FEATURE_NO_INCLUDE_DELETED),
            auth: a.nested_with_path_into(Self::FIELD_AUTH).map(|(_, a)| a),
            authz: a.nested_into(Self::FIELD_AUTHZ),
            // .nested_with_path_into::<AuthzAttr>(Self::FIELD_AUTHZ)
            // .map(|(path, mut a)| {
            //     if path {
            //         a.org = true;
            //         a.user = true;
            //     }
            //     a
            // }),
            inner: a,
        }
    }
}
impl AttrValidate for ResolverTyAttr {
    fn attr_fields(a: &Attr) -> Vec<String> {
        let f = Self::FIELDS
            .iter()
            .copied()
            .map(|f| f.to_owned())
            .filter(|f| {
                if TY_INCLUDE_DELETED.contains(&a.attr) {
                    true
                } else {
                    f != Self::FIELD_NO_INCLUDE_DELETED
                }
            });
        #[cfg(not(feature = "auth"))]
        let f = f.filter(|f| f != Self::FIELD_AUTH);
        #[cfg(not(feature = "authz"))]
        let f = f.filter(|f| f != Self::FIELD_AUTHZ);
        f.collect()
    }
}
