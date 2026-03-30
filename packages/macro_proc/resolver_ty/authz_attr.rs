use crate::prelude::*;

#[field_names]
#[derive(Clone)]
pub struct AuthzAttr {
    pub scope: String,
    pub skip_org: bool,
    pub skip_user: bool,
}
impl From<Attr> for AuthzAttr {
    fn from(a: Attr) -> Self {
        Self {
            scope: a.str_or_panic(Self::FIELD_SCOPE),
            skip_org: a.bool_should_omit(Self::FIELD_SKIP_ORG),
            skip_user: a.bool_should_omit(Self::FIELD_SKIP_USER),
        }
    }
}
impl AttrValidate for AuthzAttr {
    fn attr_fields(_: &Attr) -> Vec<String> {
        Self::FIELDS.iter().copied().map(|f| f.to_owned()).collect()
    }
}
