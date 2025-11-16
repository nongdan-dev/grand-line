use crate::prelude::*;

#[field_names]
pub struct RelationAttr {
    pub no_include_deleted: bool,
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
        attr_unwrap_or_else!(Self {
            no_include_deleted: bool,
            inner: a,
        })
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
        self.inner.field_ty().ts2()
    }
    pub fn gql_to(&self) -> Ts2 {
        ty_gql(self.to())
    }
    pub fn name(&self) -> Ts2 {
        self.inner.field_name().ts2()
    }

    pub fn key_str(&self) -> String {
        self.inner.str(Self::F_KEY).unwrap_or_else(|| {
            match self.inner.attr == RelationTy::BelongsTo {
                true => snake_str!(self.inner.field_name(), "id"),
                false => snake_str!(self.inner.field_model(), "id"),
            }
        })
    }
    pub fn through(&self) -> Ts2 {
        self.inner
            .str(Self::F_THROUGH)
            .unwrap_or_else(|| match self.inner.attr == RelationTy::ManyToMany {
                true => pascal_str!(self.inner.field_model(), "in", self.inner.field_ty()),
                false => self.bug(Self::F_THROUGH),
            })
            .ts2()
    }
    pub fn other_key(&self) -> Ts2 {
        self.inner
            .str(Self::F_OTHER_KEY)
            .unwrap_or_else(|| match self.inner.attr == RelationTy::ManyToMany {
                true => snake_str!(self.inner.field_ty(), "id"),
                false => self.bug(Self::F_OTHER_KEY),
            })
            .ts2()
    }

    fn bug(&self, k: &str) -> ! {
        let err = self
            .inner
            .errk(k, "should not access this key in this attr");
        bug!("{err}");
    }
}
