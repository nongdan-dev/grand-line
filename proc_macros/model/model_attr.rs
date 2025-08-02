use crate::prelude::*;
use field_names::FieldNames;

#[derive(FieldNames)]
pub struct ModelAttr {
    pub no_created_at: bool,
    pub no_updated_at: bool,
    pub no_deleted_at: bool,
    pub no_by_id: bool,
    pub limit_default: u64,
    pub limit_max: u64,
}
impl From<Attr> for ModelAttr {
    fn from(a: Attr) -> Self {
        attr_unwrap!(Self {
            no_created_at: bool,
            no_updated_at: bool,
            no_deleted_at: bool,
            no_by_id: bool,
            limit_default: parse,
            limit_max: parse,
        })
    }
}
impl AttrValidate for ModelAttr {
    fn attr_fields(_: &Attr) -> Vec<String> {
        Self::FIELDS.iter().map(|f| str!(f)).collect()
    }
}
