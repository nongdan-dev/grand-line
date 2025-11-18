use crate::prelude::*;

#[field_names]
pub struct ModelAttr {
    pub no_created_at: bool,
    pub no_updated_at: bool,
    pub no_deleted_at: bool,
    pub no_by_id: bool,
    #[field_names(skip)]
    pub inner: Attr,
}
impl From<Attr> for ModelAttr {
    fn from(a: Attr) -> Self {
        Self {
            no_created_at: a
                .bool(Self::FIELD_NO_CREATED_AT)
                .unwrap_or(FEATURE_NO_CREATED_AT),
            no_updated_at: a
                .bool(Self::FIELD_NO_UPDATED_AT)
                .unwrap_or(FEATURE_NO_UPDATED_AT),
            no_deleted_at: a
                .bool(Self::FIELD_NO_DELETED_AT)
                .unwrap_or(FEATURE_NO_DELETED_AT),
            no_by_id: a.bool(Self::FIELD_NO_BY_ID).unwrap_or(FEATURE_NO_BY_ID),
            inner: a,
        }
    }
}
impl AttrValidate for ModelAttr {
    fn attr_fields(_: &Attr) -> Vec<String> {
        Self::F.iter().copied().map(|f| f.to_owned()).collect()
    }
}
