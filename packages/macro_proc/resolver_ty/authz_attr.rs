use crate::prelude::*;

#[field_names]
#[derive(Clone)]
pub struct AuthzAttr {
    pub key: String,
    pub no_org: bool,
    pub no_user: bool,
}
impl From<Attr> for AuthzAttr {
    fn from(a: Attr) -> Self {
        Self {
            key: a.str_or_panic(Self::FIELD_KEY),
            no_org: a.bool_should_omit(Self::FIELD_NO_ORG),
            no_user: a.bool_should_omit(Self::FIELD_NO_USER),
        }
    }
}
impl AttrValidate for AuthzAttr {
    fn attr_fields(_: &Attr) -> Vec<String> {
        Self::FIELDS.iter().copied().map(|f| f.to_owned()).collect()
    }
}
