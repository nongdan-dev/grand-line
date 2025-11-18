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
        Self {
            no_include_deleted: a
                .bool(Self::FIELD_NO_INCLUDE_DELETED)
                .unwrap_or(FEATURE_NO_INCLUDE_DELETED),
            inner: a,
        }
    }
}
impl AttrValidate for RelationAttr {
    fn attr_fields(a: &Attr) -> Vec<String> {
        let mut f = vec![Self::FIELD_KEY];
        if a.attr == RelationTy::ManyToMany {
            f.push(Self::FIELD_THROUGH);
            f.push(Self::FIELD_OTHER_KEY);
        }
        f.iter().copied().map(|f| f.to_owned()).collect()
    }
}

impl RelationAttr {
    pub fn to(&self) -> Ts2 {
        self.inner.field_ty().ts2_or_panic()
    }
    pub fn gql_to(&self) -> Ts2 {
        ty_gql(self.to())
    }
    pub fn name(&self) -> Ts2 {
        self.inner.field_name().ts2_or_panic()
    }

    pub fn key_str(&self) -> String {
        self.inner.str(Self::FIELD_KEY).unwrap_or_else(|| {
            let field = self.inner.field_name();
            let model = self.inner.field_model();
            match self.inner.attr == RelationTy::BelongsTo {
                true => format!("{field}_id").to_snake_case(),
                false => format!("{model}_id").to_snake_case(),
            }
        })
    }
    pub fn through(&self) -> Ts2 {
        self.inner
            .str(Self::FIELD_THROUGH)
            .unwrap_or_else(|| {
                let model = self.inner.field_model();
                let ty = self.inner.field_ty();
                match self.inner.attr == RelationTy::ManyToMany {
                    true => format!("{model}_in_{ty}").to_pascal_case(),
                    false => self.bug(Self::FIELD_THROUGH),
                }
            })
            .ts2_or_panic()
    }
    pub fn other_key(&self) -> Ts2 {
        self.inner
            .str(Self::FIELD_OTHER_KEY)
            .unwrap_or_else(|| {
                let ty = self.inner.field_ty();
                match self.inner.attr == RelationTy::ManyToMany {
                    true => format!("{ty}_id").to_snake_case(),
                    false => self.bug(Self::FIELD_OTHER_KEY),
                }
            })
            .ts2_or_panic()
    }

    fn bug(&self, k: &str) -> ! {
        self.inner
            .panic_by_key(k, "should not access this key in this attr");
    }
}
