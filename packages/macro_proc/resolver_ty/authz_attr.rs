use crate::prelude::*;

#[field_names]
#[derive(Clone)]
pub struct AuthzAttr {
    pub key: String,
    pub org: bool,
    pub user: bool,
}
impl From<Attr> for AuthzAttr {
    fn from(a: Attr) -> Self {
        Self {
            key: a.str_or_panic(Self::FIELD_KEY),
            org: a.bool_should_omit(Self::FIELD_ORG),
            user: a.bool_should_omit(Self::FIELD_USER),
        }
    }
}
impl AttrValidate for AuthzAttr {
    fn attr_fields(_: &Attr) -> Vec<String> {
        Self::FIELDS.iter().copied().map(|f| f.to_owned()).collect()
    }
}
