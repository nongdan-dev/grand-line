use crate::prelude::*;

#[derive(Clone)]
pub struct RelationAttr {
    pub inner: Attr,
}

impl RelationAttr {
    pub fn new(a: Attr) -> Self {
        let a = Self { inner: a };
        if a.inner.attr == RelationTy::ManyToMany {
            return a;
        }
        for k in vec!["through", "other_key"] {
            if a.inner.has(k) {
                panic_with_location!(a.inner.msg_incorrect(k));
            }
        }
        a
    }

    pub fn to(&self) -> TokenStream2 {
        ts2!(self.inner.field_ty())
    }
    pub fn gql_to(&self) -> TokenStream2 {
        ty_gql(self.to())
    }
    pub fn name(&self) -> TokenStream2 {
        ts2!(self.inner.field_name())
    }

    pub fn key_str(&self) -> String {
        let k = "key";
        let v = self.inner.str(k);
        let v = v.unwrap_or_else(|| match self.inner.attr == RelationTy::BelongsTo {
            true => snake_str!(self.inner.field_name(), "id"),
            false => snake_str!(self.inner.field_model(), "id"),
        });
        v
    }
    pub fn through(&self) -> TokenStream2 {
        let k = "through";
        let v = self.inner.str(k);
        let v = v.unwrap_or_else(|| match self.inner.attr == RelationTy::ManyToMany {
            true => pascal_str!(self.inner.field_model(), "in", self.inner.field_ty()),
            false => self.bug(k),
        });
        ts2!(v)
    }
    pub fn other_key(&self) -> TokenStream2 {
        let k = "other_key";
        let v = self.inner.str(k);
        let v = v.unwrap_or_else(|| match self.inner.attr == RelationTy::ManyToMany {
            true => snake_str!(self.inner.field_ty(), "id"),
            false => self.bug(k),
        });
        ts2!(v)
    }

    fn bug(&self, k: &str) -> ! {
        bug!(
            self.inner
                .msgk(k, "should not access this key in this attr")
        );
    }
}
