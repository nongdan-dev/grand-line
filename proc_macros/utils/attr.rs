use crate::prelude::*;
use std::fmt::Display;
use syn::{Attribute, Field, meta::ParseNestedMeta};

fn eq_attr(a: &Attribute, attr: impl Display) -> bool {
    str!(a.path().to_token_stream()) == str!(attr)
}
fn eq_key(m: &ParseNestedMeta<'_>, key: impl Display) -> bool {
    m.path.get_ident().map(|i| str!(i)).unwrap_or_default() == str!(key)
}

pub fn has_attr(field: &Field, attr: impl Display) -> bool {
    for a in field.attrs.clone().into_iter() {
        if eq_attr(&a, &attr) {
            return true;
        }
    }
    false
}

pub fn has_attr_key(field: &Field, attr: impl Display, key: impl Display) -> bool {
    for a in field.attrs.clone().into_iter() {
        if eq_attr(&a, &attr) {
            let mut v = false;
            let _ = a.parse_nested_meta(|m| {
                if eq_key(&m, &key) {
                    v = true;
                }
                Ok(())
            });
            if v {
                return true;
            }
        }
    }
    false
}

pub fn get_attr(
    field: &Field,
    attr: impl Display,
    key: impl Display,
    default: impl Display,
) -> String {
    for a in field.attrs.clone().into_iter() {
        if eq_attr(&a, &attr) {
            let mut v = None;
            let _ = a.parse_nested_meta(|m| {
                if eq_key(&m, &key) && m.input.peek(syn::Token![=]) {
                    v = Some(str!(m.value()?));
                }
                Ok(())
            });
            if let Some(v) = v {
                return v;
            }
        }
    }
    str!(default)
}

pub fn must_get_attr(
    model: impl Display,
    field: &Field,
    attr: impl Display,
    key: impl Display,
    default: impl Display,
) -> String {
    let r = get_attr(&field, &attr, &key, &default);
    if r == "" {
        let field = field.ident.to_token_stream();
        panic!(
            "{}.{} attr={} key={} error: not found",
            model, field, attr, key
        );
    }
    r
}

pub trait MustGetAttrImpl {
    fn impl_attr_model(&self) -> &dyn Display;
    fn impl_attr_field(&self) -> &Field;
    fn impl_attr_name(&self) -> &dyn Display;

    fn attr(&self, key: impl Display, default: impl Display) -> TokenStream2 {
        ts2!(self.attr_str(key, default))
    }
    fn attr_str(&self, key: impl Display, default: impl Display) -> String {
        let model = self.impl_attr_model();
        let field = self.impl_attr_field();
        let attr = self.impl_attr_name();
        must_get_attr(model, field, attr, key, default)
    }
    fn attr_panic(&self, err: impl Display) -> ! {
        let model = self.impl_attr_model();
        let field = self.impl_attr_field().ident.to_token_stream();
        let attr = self.impl_attr_name();
        panic!("{}.{} attr={} {}", model, field, attr, err);
    }
}
