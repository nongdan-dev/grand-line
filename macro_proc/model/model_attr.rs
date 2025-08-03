use crate::prelude::*;

#[field_names]
pub struct ModelAttr {
    pub no_created_at: bool,
    pub no_updated_at: bool,
    pub no_deleted_at: bool,
    pub no_by_id: bool,
    pub limit_default: u64,
    pub limit_max: u64,
    #[allow(dead_code)]
    #[field_names(skip)]
    pub inner: Attr,
}
impl From<Attr> for ModelAttr {
    fn from(a: Attr) -> Self {
        attr_unwrap_or_else!(Self {
            no_created_at: bool,
            no_updated_at: bool,
            no_deleted_at: bool,
            no_by_id: bool,
            limit_default: parse,
            limit_max: parse,
            inner: a,
        })
    }
}
impl AttrValidate for ModelAttr {
    fn attr_fields(_: &Attr) -> Vec<String> {
        Self::F.iter().map(|f| s!(f)).collect()
    }
}
