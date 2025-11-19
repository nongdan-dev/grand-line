use crate::prelude::*;

#[field_names]
#[derive(Clone)]
pub struct AuthzAttr {
    pub org: bool,
    pub user: bool,
    pub key: Option<String>,
}
impl From<Attr> for AuthzAttr {
    fn from(a: Attr) -> Self {
        Self {
            org: a.bool_should_omit(Self::FIELD_ORG),
            user: a.bool_should_omit(Self::FIELD_USER),
            key: a.str(Self::FIELD_KEY),
        }
    }
}
impl AttrValidate for AuthzAttr {
    fn attr_fields(_: &Attr) -> Vec<String> {
        Self::FIELDS.iter().copied().map(|f| f.to_owned()).collect()
    }
}
