use crate::prelude::*;

#[field_names]
#[derive(Clone)]
pub struct AuthAttr {
    pub unauthenticated: bool,
}
impl From<Attr> for AuthAttr {
    fn from(a: Attr) -> Self {
        Self {
            unauthenticated: a.bool_should_omit(Self::FIELD_UNAUTHENTICATED),
        }
    }
}
impl AttrValidate for AuthAttr {
    fn attr_fields(_: &Attr) -> Vec<String> {
        Self::FIELDS.iter().copied().map(|f| f.to_owned()).collect()
    }
}
