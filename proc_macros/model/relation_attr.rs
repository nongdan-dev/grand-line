use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct RelationAttr {
    pub a: Attr,
}

impl RelationAttr {
    pub fn new(a: Attr) -> Self {
        let a = Self { a };
        if a.a.attr == RelationTy::ManyToMany {
            return a;
        }
        for k in vec!["through", "other_key"] {
            if a.a.str(k).is_some() {
                a.a.panic_incorrect(k)
            }
        }
        a
    }

    pub fn to(&self) -> TokenStream2 {
        ts2!(self.a.field_ty())
    }
    pub fn gql_to(&self) -> TokenStream2 {
        ty_gql(self.to())
    }
    pub fn name(&self) -> TokenStream2 {
        ts2!(self.a.field_name())
    }

    pub fn key_str(&self) -> String {
        let k = "key";
        let v = self.a.str(k);
        let v = v.unwrap_or_else(|| match self.a.attr == RelationTy::BelongsTo {
            true => snake_str!(self.a.field_name(), "id"),
            false => snake_str!(self.a.field_model(), "id"),
        });
        v
    }
    pub fn through(&self) -> TokenStream2 {
        let k = "through";
        let v = self.a.str(k);
        let v = v.unwrap_or_else(|| match self.a.attr == RelationTy::ManyToMany {
            true => pascal_str!(self.a.field_model(), "in", self.a.field_ty()),
            false => self.panic_framework_bug(k),
        });
        ts2!(v)
    }
    pub fn other_key(&self) -> TokenStream2 {
        let k = "other_key";
        let v = self.a.str(k);
        let v = v.unwrap_or_else(|| match self.a.attr == RelationTy::ManyToMany {
            true => snake_str!(self.a.field_ty(), "id"),
            false => self.panic_framework_bug(k),
        });
        ts2!(v)
    }

    fn panic_framework_bug(&self, k: &str) -> ! {
        let err = strf!("trying to get `{}` in `{}`", k, self.a.attr);
        self.a.panic_framework_bug(&err)
    }
}
