use crate::prelude::*;

#[field_names]
#[derive(Clone)]
pub struct RelationAttr {
    #[field_names(skip)]
    pub inner: Attr,
    #[field_names(virt)]
    key: !,
    #[field_names(virt)]
    through: !,
    #[field_names(virt)]
    other_key: !,
}
impl From<Attr> for RelationAttr {
    fn from(a: Attr) -> Self {
        Self { inner: a }
    }
}
impl AttrValidate for RelationAttr {
    fn attr_fields(a: &Attr) -> Vec<String> {
        let mut f = vec![Self::F_KEY];
        if a.attr == RelationTy::ManyToMany {
            f.push(Self::F_THROUGH);
            f.push(Self::F_OTHER_KEY);
        }
        f.iter().map(|f| s!(f)).collect()
    }
}

impl RelationAttr {
    pub fn to(&self) -> Ts2 {
        ts2!(self.inner.field_ty())
    }
    pub fn gql_to(&self) -> Ts2 {
        ty_gql(self.to())
    }
    pub fn name(&self) -> Ts2 {
        ts2!(self.inner.field_name())
    }

    pub fn key_str(&self) -> String {
        let v = self.inner.str(Self::F_KEY);
        let v = v.unwrap_or_else(|| match self.inner.attr == RelationTy::BelongsTo {
            true => snake_str!(self.inner.field_name(), "id"),
            false => snake_str!(self.inner.field_model(), "id"),
        });
        v
    }
    pub fn through(&self) -> Ts2 {
        let v = self.inner.str(Self::F_THROUGH);
        let v = v.unwrap_or_else(|| match self.inner.attr == RelationTy::ManyToMany {
            true => pascal_str!(self.inner.field_model(), "in", self.inner.field_ty()),
            false => self.bug(Self::F_THROUGH),
        });
        ts2!(v)
    }
    pub fn other_key(&self) -> Ts2 {
        let v = self.inner.str(Self::F_OTHER_KEY);
        let v = v.unwrap_or_else(|| match self.inner.attr == RelationTy::ManyToMany {
            true => snake_str!(self.inner.field_ty(), "id"),
            false => self.bug(Self::F_OTHER_KEY),
        });
        ts2!(v)
    }

    fn bug(&self, k: &str) -> ! {
        let err = self
            .inner
            .errk(k, "should not access this key in this attr");
        bug!(err);
    }
}
