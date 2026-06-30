use crate::prelude::*;

#[field_names]
pub struct ResolverTyAttr {
    pub tx: bool,
    pub ctx: bool,
    pub include_deleted: bool,
    pub auth: Option<AuthAttr>,
    pub authz: Option<AuthzAttr>,
    pub authz_row: bool,
    #[field_names(skip)]
    pub inner: Attr,
}

impl ResolverTyAttr {
    pub fn has_auth(&self) -> bool {
        let auth = self.auth.as_ref();
        let auth = auth.is_some() && !auth.is_some_and(|v| v.unauthenticated);
        let authz = self.authz.as_ref();
        let authz = authz.is_some();
        auth || authz
    }
}

impl TryFrom<Attr> for ResolverTyAttr {
    type Error = SynErr;
    fn try_from(a: Attr) -> SynRes<Self> {
        Ok(Self {
            tx: a.bool(Self::FIELD_TX)?.unwrap_or(FEATURE_RESOLVER_TX),
            ctx: a.bool(Self::FIELD_CTX)?.unwrap_or(FEATURE_RESOLVER_CTX),
            include_deleted: a
                .bool(Self::FIELD_INCLUDE_DELETED)?
                .unwrap_or(FEATURE_RESOLVER_INCLUDE_DELETED),
            auth: a.nested_with_path_into::<AuthAttr>(Self::FIELD_AUTH)?.map(|(_, a)| a),
            authz: a.nested_into::<AuthzAttr>(Self::FIELD_AUTHZ)?,
            authz_row: a.bool(Self::FIELD_AUTHZ_ROW)?.unwrap_or(FEATURE_RESOLVER_AUTHZ_ROW),
            inner: a,
        })
    }
}

impl AttrValidate for ResolverTyAttr {
    fn attr_fields(a: &Attr) -> Vec<String> {
        let f = Self::FIELDS.iter().copied().map(|f| f.to_owned()).filter(|f| {
            if TY_INCLUDE_DELETED.contains(&a.attr) {
                true
            } else {
                f != Self::FIELD_INCLUDE_DELETED
            }
        });
        #[cfg(not(feature = "auth"))]
        let f = f.filter(|f| f != Self::FIELD_AUTH);
        #[cfg(not(feature = "authz"))]
        let f = f.filter(|f| f != Self::FIELD_AUTHZ);
        f.collect()
    }
}
